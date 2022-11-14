use std::fs;
mod wilma;
mod schedule;

fn get_schedule() -> Vec<wilma::Schedule> {

    let credentials_data = fs::read_to_string(format!("{}/.config/wilma-tui/config", std::env::var("HOME").unwrap())).unwrap();
    let credentials: Vec<&str> = credentials_data.lines().collect();

    let user = credentials[0];
    let password = credentials[1];
    let base_url = credentials[2];

 
    reqwest::blocking::get(base_url).expect("base_url is invalid");


    let mut download = false;
    let dir = format!("{}/.local/share/wilma-bar", std::env::var("HOME").unwrap());
    let date_now = chrono::Local::now().naive_local().date();
    let dir_exist = fs::metadata(&dir).is_ok();
    if dir_exist == false {
        fs::create_dir_all(&dir).unwrap();
    }

    let time_exist = fs::metadata(format!("{}/time", &dir)).is_ok();
    if !time_exist {
        download = true;
    }
    else {
        let date = chrono::NaiveDate::parse_from_str(&fs::read_to_string(format!("{}/time",&dir)).unwrap().trim(), "%Y-%m-%d");
        //let date = chrono::NaiveDate::parse_from_str("2022-08-24", "%Y-%m-%d");
        //println!("{} {}", date_now, date.unwrap());
        if date_now > date.unwrap() {
            download = true;
        }
    }
    if download == true {
        fs::write(format!("{}/time", &dir), date_now.format("%Y-%m-%d").to_string()).unwrap();

        let login_info = wilma::LoginInfo::login(user, password, base_url).unwrap();
        let data = wilma::Schedule::new(&login_info.wilma2sid, &login_info.formkey, &base_url);
        //println!("data {:#?}", data);
        fs::write(format!("{}/data.json", &dir), serde_json::to_string(&data).unwrap()).unwrap();
    }
    let data: Vec<wilma::Schedule> = serde_json::from_str(&fs::read_to_string(format!("{}/data.json", &dir)).unwrap()).unwrap();
    return data;
}

fn main() {
    let data = get_schedule();

    let time_now = chrono::Local::now().time();
    //let time_now = chrono::NaiveTime::parse_from_str("11:50","%H:%M").unwrap();
    let mut message = String::new();
    for lesson in data {
        //println!("{:#?}", lesson);
        let (start_str,end_str) = lesson.time.split_once('-').unwrap();
        let start = chrono::NaiveTime::parse_from_str(start_str,"%H:%M").unwrap();
        let end = chrono::NaiveTime::parse_from_str(end_str,"%H:%M").unwrap();
        if time_now > start && time_now < end {
            let mut minutes = (end-time_now).num_minutes();
            let hours = minutes / 60;
            minutes = minutes % 60 + 1;
            if hours > 0 {
                message = format!("{} ends in {} h and {} m", lesson.name, hours, minutes);
            }
            else {
                message = format!("{} ends in {} m", lesson.name, minutes);
            }
            break;
        }
        if time_now < start && time_now < end {
            let mut minutes = (start-time_now).num_minutes();
            let hours = minutes / 60;
            minutes = minutes % 60;
            if hours > 0 {
                message = format!("{} starts in {} h and {} m", lesson.name, hours, minutes);
            }
            else {
                message = format!("{} starts in {} m", lesson.name, minutes);
            }
            break;
        }
        else {
            message = "No lessons today :)".to_string();
        }
    }
    println!("{}", message);
}
