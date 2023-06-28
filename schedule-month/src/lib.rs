use anyhow::Result;
use chrono::{Datelike, Month, NaiveDate, Weekday};
use rand::{seq::SliceRandom, thread_rng};
use spin_sdk::{
    config,
    http::{Params, Request, Response},
    http_component, http_router,
    key_value::{Error, Store},
};
use std::collections::HashMap;
use std::str::FromStr;

const YEAR: u16 = 2023; // TODO replace with call to chrono for current year

/// A api for managing the monthly triage rotation
#[http_component]
fn handle_schedule_month(req: Request) -> Result<Response> {
    let router = http_router! {
        GET "/schedule/:month" => api::get_schedule,
        POST "/schedule/:month" => api::create_schedule,
        DELETE "/schedule/:month" => api::delete_schedule
    };
    router.handle(req)
}

mod api {
    use super::*;

    // /schedule/:month
    pub fn get_schedule(_req: Request, params: Params) -> anyhow::Result<Response> {
        let month_str = params.get("month").expect("MONTH");

        let month = match Month::from_str(month_str) {
            Ok(month) => month,
            Err(_) => {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(Some(format!("Invalid month {month_str}").into()))
                    .unwrap());
            }
        };

        let store = Store::open_default()?;
        match store.get(month.name()) {
            Ok(val) => {
                let sched_str = std::str::from_utf8(&val).unwrap().to_string();
                return Ok(http::Response::builder()
                    .status(http::StatusCode::OK)
                    .header("Content-Type", "application/json")
                    .body(Some(format!("{}", sched_str).into()))
                    .unwrap());
            }
            Err(e) => {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Some(
                        format!("Error getting schedule for month {}: {:?}", month.name(), e)
                            .into(),
                    ))
                    .unwrap());
            }
        }
    }

    pub fn create_schedule(_req: Request, params: Params) -> anyhow::Result<Response> {
        let month_str = params.get("month").expect("MONTH");

        let month = match Month::from_str(month_str) {
            Ok(month) => month,
            Err(_) => {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(Some(format!("Invalid month {month_str}").into()))
                    .unwrap());
            }
        };

        match assign_schedule(month) {
            Ok(_) => Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .body(Some(
                    format!("Successfully created schedule for month {}", month.name()).into(),
                ))
                .unwrap()),
            Err(e) => Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Some(
                    format!("Error creating schedule for month {:?}", e).into(),
                ))
                .unwrap()),
        }
    }

    pub fn delete_schedule(_req: Request, params: Params) -> anyhow::Result<Response> {
        let month_str = params.get("month").expect("MONTH");

        let month = match Month::from_str(month_str) {
            Ok(month) => month,
            Err(_) => {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::BAD_REQUEST)
                    .body(Some(format!("Invalid month {month_str}").into()))
                    .unwrap());
            }
        };

        let store = Store::open_default()?;
        match store.delete(month.name()) {
            Ok(_) => Ok(http::Response::builder()
                .status(http::StatusCode::OK)
                .body(Some(
                    format!("Schedule for month {} has been deleted", month_str).into(),
                ))
                .unwrap()),
            Err(e) => Ok(http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Some(format!("Error deleting schedule {:?}", e).into()))
                .unwrap()),
        }
    }
}

fn assign_schedule(month: Month) -> Result<(), anyhow::Error> {
    let store = Store::open_default()?;
    let schedule = match store.get(month.name()) {
        Ok(_) => return Err(anyhow::anyhow!("month already scheduled")),
        Err(Error::NoSuchKey) => build_schedule(month, &store)?,
        Err(e) => return Err(anyhow::anyhow!("error accessing storage: {}", e)), // TODO make better?
    };

    store_schedule(month.name(), schedule, &store).unwrap();

    return Ok(());
}

fn last_day_of_month(month: Month) -> Result<NaiveDate, anyhow::Error> {
    // TODO: deal with December
    match NaiveDate::from_ymd_opt(
        YEAR.into(),
        month.number_from_month(),
        days_in_month(month).into(),
    ) {
        Some(date) => return Ok(date),
        None => return Err(anyhow::anyhow!("Unable to get last date of previous month")),
    }
}

fn get_last_pair(month: Month, store: &Store) -> Option<String> {
    match store.get(month.name()) {
        Ok(val) => {
            let sched: HashMap<String, String> = serde_json::from_slice(&val).unwrap();
            match last_day_of_month(month) {
                Ok(date) => match sched.get(&date.to_string()) {
                    Some(pair) => return Some(pair.clone()),
                    None => return None,
                },
                Err(e) => {
                    println!("{}", e);
                    return None;
                }
            }
        }
        Err(e) => {
            println!("Error retrieving previous month schedule: {}", e);
            return None;
        }
    }
}

fn build_schedule(month: Month, store: &Store) -> Result<HashMap<u32, String>> {
    let month_number = month.number_from_month();
    let first_day_of_month = match NaiveDate::from_ymd_opt(YEAR.into(), month_number, 1) {
        Some(day) => day,
        None => panic!("deal with None"),
    };

    let mut pair = match get_last_pair(month.pred(), &store) {
        Some(pair) => pair,
        None => String::from("Joey and Janice"),
    };

    let mut schedule = HashMap::new();

    for date in first_day_of_month
        .iter_days()
        .take(days_in_month(month).into())
    {
        if date.weekday() == Weekday::Mon {
            pair = new_pair();
        }
        schedule.insert(date.day(), pair.clone());
    }

    return Ok(schedule);
}

fn days_in_month(month: Month) -> u8 {
    let days = match month {
        Month::January
        | Month::March
        | Month::May
        | Month::July
        | Month::August
        | Month::October
        | Month::December => 31,
        Month::April | Month::June | Month::September | Month::November => 30,
        Month::February => {
            if leap_year(YEAR) {
                29
            } else {
                28
            }
        }
    };

    return days;
}

fn leap_year(year: u16) -> bool {
    // leap years are divisible by 4 and not divisible by 100 unless they are also divisible by 400
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

fn store_schedule(month: &str, schedule: HashMap<u32, String>, store: &Store) -> Result<(), Error> {
    let json_string =
        serde_json::to_string(&schedule).expect("Failed to serialize HashMap to JSON");

    return store.set(month, json_string);
}

fn new_pair() -> String {
    const MAINTAINERS_CONFIG_VARIABLE: &str = "maintainers";

    let maintainers =
        config::get(MAINTAINERS_CONFIG_VARIABLE).expect("unable to parse maintainers config");
    let maintainers_list: Vec<&str> = maintainers.split(',').map(|s| s.trim()).collect();

    let mut rng = thread_rng();
    let chosen = maintainers_list.choose_multiple(&mut rng, 2);

    let rotation: Vec<String> = chosen.into_iter().map(|c| c.to_string()).collect();
    rotation.join(" and ")
}
