use crate::overview;
use chrono::Datelike;
use serde_derive::Deserialize;
use serde_derive::Serialize;

fn find_str(page: &String, start: &str) -> Option<String> {
    let first = page.find(&start)?;
    let end = page[first+start.len()..first+100].find("\"")?;
    Some(page[first+start.len()..first+start.len()+end].to_string())
}

pub struct LoginInfo {
    pub formkey: String,
    pub wilma2sid: String,
    pub slug: Option<String>
}

impl LoginInfo {
    pub fn login(user: &str, password: &str, base_url: &str) -> Result<LoginInfo, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let res = client.get(format!("{}/index_json", base_url))
            .send().expect("Can't /index_json");
        let obj: serde_json::Value = serde_json::from_str(&res.text().unwrap()).expect("Can't parse /index_json");
        let mut login_id: String = obj["SessionID"].to_string(); login_id.pop(); login_id.remove(0);
        if login_id == "null" { panic!("no session id") }
        let params = [
            ("Login", user),
            ("Password", password),
            ("SESSIONID", &login_id)
        ];
        let res2 = client.post(format!("{}/login", base_url))
            .form(&params)
            .send().expect("Can't /login");
        //if res2.text()?.is_empty() == true { panic!("Can't login"); }
        let cookie = res2.cookies().last().unwrap();
        let wilma2sid_ = String::from(cookie.value());
        if wilma2sid_ == "" { panic!("No wilma2sid, probably wrong credentials"); }

        let res3 = client.get(base_url)
            .header("Cookie", format!("Wilma2SID={}", wilma2sid_))
            .send().expect("Can't baseurl");


        let page: String = res3.text()?;

        let formkey_ = find_str(&page, "formkey\" value=\"");
        let slug_ = find_str(&page, "presentation\"><a href=\"/!");

        Ok(LoginInfo { wilma2sid: wilma2sid_, formkey: formkey_.unwrap(), slug: slug_ })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schedule {
    pub name: String,
    pub teacher: String,
    pub room: String,
    pub time: String
}

impl Schedule {
    pub fn new(wilma2sid: &str, formkey: &str,
               slug: &Option<String>, base_url: &str) -> Result<Vec<Schedule>, Box<dyn std::error::Error>> {
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        let url;
        match slug {
            Some(x) => url = format!("{}/!{}/overview", base_url, x),
            None => url = format!("{}/overview", base_url)
        }

        let day = chrono::Local::today().format("%Y-%m-%d").to_string();

        let params = [
            ("date", day.as_str()),
            ("getfullmonth", "true"),
            ("formkey", formkey)
        ];
        let root = client.post(url)
            .header("Cookie", format!("Wilma2SID={}", wilma2sid))
            .form(&params)
            .send().expect("Can't get schedule").json::<overview::Root>().expect("Can't parse schedule");

        let mut today_sche: Vec<Schedule> = Vec::new();

        let current_day = chrono::offset::Local::now().date().weekday().number_from_monday();
        
        for sch in root.schedule {
            let x = Schedule { name: sch.groups[0].caption.clone(),
            teacher: sch.groups[0].teachers.as_ref().unwrap_or(
                &vec!(overview::Teacher { id: 0, caption: "".to_string(),
                long_caption: "".to_string(), schedule_visible: false
            }))[0].long_caption.clone(),
            room: sch.groups[0].rooms.as_ref().unwrap_or(
                &vec!(overview::Room { id: 0, caption: "".to_string(),
                long_caption: "".to_string(), schedule_visible: false
            }))[0].caption.clone(),
            time: format!("{}–{}", sch.start, sch.end)};
            if sch.groups[0].id != None && sch.day == current_day as i64{
                today_sche.push(x);
            }
        }


        Ok(today_sche)
    }
}
