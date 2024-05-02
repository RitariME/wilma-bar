use crate::schedule;
use chrono::Datelike;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use reqwest::header::{HeaderMap, COOKIE};

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
            .cookie_store(true)
            .build()?;
        let res = client.get(format!("{}/index_json", base_url))
            .send().expect("Can't /index_json");
        let obj: serde_json::Value = serde_json::from_str(&res.text().unwrap()).expect("Can't parse /index_json");
        let mut login_id: String = obj["SessionID"].to_string(); login_id.pop(); login_id.remove(0);
        if login_id == "null" { panic!("no session id") }
        let params = [
            ("Login", user),
            ("Password", password),
            ("submit", "Kirjaudu+sisään"),
            ("SESSIONID", &login_id)
        ];
        let mut headers = HeaderMap::new();
        headers.insert(
            COOKIE,
            format!("Wilma2LoginID={}", login_id).parse().unwrap(),
        );
        let res2 = client.post(format!("{}/login", base_url))
            .headers(headers)
            .form(&params)
            .send().expect("Can't /login");
        let cookie = res2.cookies().last().unwrap();

        let wilma2sid_ = String::from(cookie.value());
        if wilma2sid_ == "" { panic!("No wilma2sid, probably wrong credentials"); }

        let res3 = client.get(base_url)
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
    pub time: String
}

impl Schedule {
    pub fn new(wilma2sid: &str, formkey: &str, base_url: &str) -> Vec<Schedule> {
        let day = chrono::Local::today().format("%d-%m-%Y").to_string();
        let mut today_sche: Vec<Schedule> = Vec::new();
        let current_day = chrono::offset::Local::now().date().weekday().number_from_monday();
        let primusid = formkey.split(':').collect::<Vec<&str>>()[1];

        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build().unwrap();

        let root = client.get(format!("{}/schedule/export/students/{}?date={}", base_url, primusid, day))
            .header("Cookie", format!("Wilma2SID={}", wilma2sid))
            .send().expect("Can't get schedule").json::<schedule::Root>().expect("Can't parse schedule");
        for v in root.schedule {
            if v.day.unwrap_or(100) as u32 == current_day {
                let schedule = Schedule {
                    time: format!("{}-{}", v.start.unwrap_or("00:00".to_string()), v.end.unwrap_or("00:00".to_string())),
                    name: v.groups[0].short_caption.clone().unwrap_or("no".to_string()),
                };
                today_sche.push(schedule);
            }
        }
        today_sche

    }
}
