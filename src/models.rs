use chrono::naive::NaiveDateTime;

use crate::schema::fanspeed;

#[derive(Debug, Queryable)]
pub struct Fanspeed {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub fan_name: String,
    pub rpm: i32,
}

#[derive(Debug, Insertable, Queryable)]
#[table_name = "fanspeed"]
pub struct NewFanspeed {
    pub timestamp: NaiveDateTime,
    pub fan_name: String,
    pub rpm: i32,
}
