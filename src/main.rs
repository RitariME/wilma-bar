use std::fs;
mod wilma;
mod overview;

fn main() {
    let data = fs::read_to_string(format!("{}/.config/wilma-tui/config", std::env::var("HOME").unwrap())).unwrap();
    let credentials: Vec<&str> = data.lines().collect();

    let user = credentials[0];
    let password = credentials[1];
    let base_url = credentials[2];

    let login_info = wilma::LoginInfo::login(user, password, base_url).unwrap();
    let data = wilma::Schedule::new(&login_info.wilma2sid, &login_info.formkey, &login_info.slug, &base_url).unwrap();
    let time_now = chrono::Local::now().time();
    //let time_now = chrono::NaiveTime::parse_from_str("11:50","%H:%M").unwrap();
    let mut message = String::new();
    for lesson in data {
        let (start_str,end_str) = lesson.time.split_once('–').unwrap();
        let start = chrono::NaiveTime::parse_from_str(start_str,"%H:%M").unwrap();
        let end = chrono::NaiveTime::parse_from_str(end_str,"%H:%M").unwrap();
        if time_now > start && time_now < end {
            let mut minutes = (end-time_now).num_minutes();
            let hours = minutes / 60;
            minutes = minutes % 60;
            if hours > 0 {
                message = format!("{} Lesson ends in {} hours and {} minutes", lesson.name, hours, minutes);
            }
            else {
                message = format!("{} Lesson ends in {} minutes", lesson.name, minutes);
            }
            break;
        }
        if time_now < start && time_now < end {
            let mut minutes = (start-time_now).num_minutes();
            let hours = minutes / 60;
            minutes = minutes % 60;
            if hours > 0 {
                message = format!("{} Lesson starts in {} hours and {} minutes", lesson.name, hours, minutes);
            }
            else {
                message = format!("{} Lesson starts in {} minutes", lesson.name, minutes);
            }
            break;
        }
        else {
            message = "No lessons today :)".to_string();
        }
    }
    println!("{}", message);
}
