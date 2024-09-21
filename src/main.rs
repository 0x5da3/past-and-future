#![allow(non_snake_case)]

use chrono::{FixedOffset, NaiveDate, TimeDelta, TimeZone};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

const BIRTHDAY: &str = "Birthday";
const DEATHANNIV: &str = "DeathAnniv";

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut birth_day = use_signal(|| "2000-01-01".to_string());
    let mut death_anniv_day = use_signal(|| "2040-01-01".to_string());

    let today = (chrono::Utc::now() + FixedOffset::east_opt(9 * 60 * 60).unwrap()).date_naive();
    info!("{birth_day}");
    let chrono_bd = chrono::NaiveDate::parse_from_str(&birth_day.read(), "%Y-%m-%d").unwrap();
    let chrono_dad =
        chrono::NaiveDate::parse_from_str(&death_anniv_day.read(), "%Y-%m-%d").unwrap();

    let past_time = rt_time_str(today, chrono_bd);
    let future_time = rt_time_str(chrono_dad, today);

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        form {
            onsubmit: move |event| {
                info!("{event:#?}");
                for (name, v) in event.values() {
                    match name.as_str() {
                        BIRTHDAY => {
                            let v = v.as_value();
                            let date = chrono::NaiveDate::parse_from_str(&v, "%Y-%m-%d")
                                .unwrap();
                            birth_day.set(date.to_string())
                        }
                        DEATHANNIV => {
                            let v = v.as_value();
                            let date = chrono::NaiveDate::parse_from_str(&v, "%Y-%m-%d")
                                .unwrap();
                            death_anniv_day.set(date.to_string())
                        }
                        _ => {}
                    }
                }
            },
            h4 { "生年月日" }
            input { r#type: "date", name: "Birthday", value: "{birth_day}" }
            h4 { "今日" }
            h5 { "{today:?}" }

            h4 { "命日" }
            input { r#type: "date", name: "DeathAnniv", value: "2040-01-01" }
            br {}
            hr {}
            button { r#type: "submit", "送信" }
        }

        h3 { "これまで生きてきた時間" }
        div { "{past_time}" }
        h3 { "これから生きていく時間" }
        div { "{future_time}" }
    }
}

fn get_today() {}

fn rt_time_str(left_item: NaiveDate, right_item: NaiveDate) -> String {
    let time: TimeDelta = left_item - right_item;
    let secs = time.num_seconds();
    let minutes = time.num_minutes();
    let hour = time.num_hours();
    let days = time.num_days();
    let weeks = time.num_weeks();
    let times = format!("{weeks}週 {days}日 {hour}時間 {minutes}分 {secs}秒");
    times
}
