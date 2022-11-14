use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "Schedule")]
    pub schedule: Vec<Schedule>,
    #[serde(rename = "Terms")]
    pub terms: Vec<Term>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    #[serde(rename = "ReservationID")]
    pub reservation_id: Option<i64>,
    #[serde(rename = "ScheduleID")]
    pub schedule_id: Option<i64>,
    #[serde(rename = "Day")]
    pub day: Option<i64>,
    #[serde(rename = "Start")]
    pub start: Option<String>,
    #[serde(rename = "End")]
    pub end: Option<String>,
    #[serde(rename = "Color")]
    pub color: Option<String>,
    #[serde(rename = "X1")]
    pub x1: Option<i64>,
    #[serde(rename = "Y1")]
    pub y1: Option<i64>,
    #[serde(rename = "X2")]
    pub x2: Option<i64>,
    #[serde(rename = "Y2")]
    pub y2: Option<i64>,
    #[serde(rename = "Class")]
    pub class: Option<String>,
    #[serde(rename = "AllowEdit")]
    pub allow_edit: Option<bool>,
    #[serde(rename = "AllowAddMoveRemove")]
    pub allow_add_move_remove: Option<bool>,
    #[serde(rename = "Groups")]
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "Id")]
    pub id: Option<i64>,
    #[serde(rename = "CourseId")]
    pub course_id: Option<i64>,
    #[serde(rename = "ShortCaption")]
    pub short_caption: Option<String>,
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "FullCaption")]
    pub full_caption: Option<String>,
    #[serde(rename = "Class")]
    pub class: Option<String>,
    #[serde(rename = "Teachers")]
    pub teachers: Vec<Teacher>,
    #[serde(rename = "Rooms")]
    pub rooms: Vec<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    #[serde(rename = "Id")]
    pub id: Option<i64>,
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "LongCaption")]
    pub long_caption: Option<String>,
    #[serde(rename = "ScheduleVisible")]
    pub schedule_visible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(rename = "Id")]
    pub id: Option<i64>,
    #[serde(rename = "Caption")]
    pub caption: Option<String>,
    #[serde(rename = "LongCaption")]
    pub long_caption: Option<String>,
    #[serde(rename = "ScheduleVisible")]
    pub schedule_visible: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "StartDate")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate")]
    pub end_date: Option<String>,
}
