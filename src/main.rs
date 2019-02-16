#[macro_use]
extern crate diesel;

mod fan;
mod models;
mod schema;

use std::env;
use std::io;
use std::thread;
use std::time::Duration;

use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

use fan::lm_sensors;
use models::{Fanspeed, NewFanspeed};

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() -> Result<(), io::Error> {
    let connection = establish_connection();

    loop {
        use schema::fanspeed::dsl::*;
        // Get timestamp
        let ts = Utc::now().naive_utc();
        // Get fan measurements
        let measurements = lm_sensors("thinkpad-isa-0000")?;

        // Insert record into database and get the row that was inserted
        let new_fanspeed: Fanspeed = diesel::insert_into(fanspeed)
            .values(
                measurements
                    .into_iter()
                    .map(|(name, rpm_)| NewFanspeed {
                        timestamp: ts,
                        fan_name: name,
                        rpm: rpm_,
                    })
                    .collect::<Vec<NewFanspeed>>(),
            )
            .get_result(&connection)
            .expect("Error saving fan speed");
        // Print the row
        println!("Added {:?} to database", new_fanspeed);
        // Sleep
        thread::sleep(Duration::from_secs(1));
    }
}
