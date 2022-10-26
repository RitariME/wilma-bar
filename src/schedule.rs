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
    pub reservation_id: i64,
    #[serde(rename = "ScheduleID")]
    pub schedule_id: i64,
    #[serde(rename = "Day")]
    pub day: i64,
    #[serde(rename = "Start")]
    pub start: String,
    #[serde(rename = "End")]
    pub end: String,
    #[serde(rename = "Color")]
    pub color: String,
    #[serde(rename = "X1")]
    pub x1: i64,
    #[serde(rename = "Y1")]
    pub y1: i64,
    #[serde(rename = "X2")]
    pub x2: i64,
    #[serde(rename = "Y2")]
    pub y2: i64,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "AllowEdit")]
    pub allow_edit: bool,
    #[serde(rename = "AllowAddMoveRemove")]
    pub allow_add_move_remove: bool,
    #[serde(rename = "Groups")]
    pub groups: Vec<Group>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "CourseId")]
    pub course_id: i64,
    #[serde(rename = "ShortCaption")]
    pub short_caption: String,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "FullCaption")]
    pub full_caption: String,
    #[serde(rename = "Class")]
    pub class: String,
    #[serde(rename = "Teachers")]
    pub teachers: Vec<Teacher>,
    #[serde(rename = "Rooms")]
    pub rooms: Vec<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "LongCaption")]
    pub long_caption: String,
    #[serde(rename = "ScheduleVisible")]
    pub schedule_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Room {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Caption")]
    pub caption: String,
    #[serde(rename = "LongCaption")]
    pub long_caption: String,
    #[serde(rename = "ScheduleVisible")]
    pub schedule_visible: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StartDate")]
    pub start_date: String,
    #[serde(rename = "EndDate")]
    pub end_date: String,
}
