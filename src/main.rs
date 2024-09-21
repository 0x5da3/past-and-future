#![allow(non_snake_case)]

use chrono::{TimeDelta, TimeZone};
use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

const YEAR: &str = "year";
const MONTH: &str = "month";
const DAY: &str = "day";

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut year = use_signal(|| 2000i32);
    let mut month = use_signal(|| 1u32);
    let mut day = use_signal(|| 1u32);
    let today = chrono::Utc::now().date_naive();
    let birth_day = chrono::Utc
        .with_ymd_and_hms(*year.read(), *month.read(), *day.read(), 0, 0, 0)
        .single()
        .unwrap()
        .date_naive();
    let past_time: TimeDelta = today - birth_day;
    let secs = past_time.num_seconds();
    let minutes = past_time.num_minutes();
    let hour = past_time.num_hours();
    let days = past_time.num_days();
    let weeks = past_time.num_weeks();

    let t = format!("{weeks}週 {days}日 {hour}時間 {minutes}分 {secs}秒");

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        h2 { "生年月日" }
        form {
            onsubmit: move |event| {
                info!("{event:#?}");
                for (name, v) in event.values() {
                    let v = v.as_value();
                    match name.as_str() {
                        YEAR => year.set(v.parse::<i32>().unwrap()),
                        MONTH => month.set(v.parse::<u32>().unwrap()),
                        DAY => day.set(v.parse::<u32>().unwrap()),
                        _ => {}
                    }
                }
            },
            input { r#type: "date", name: "birth_day" }
            input { r#type: "date", name: "future" }
            button { r#type: "submit", "送信" }
        }
        div { "{today:?}" }
        div { "{t}" }
        div {
            h2 { "これから生きていく時間" }
        }
    }
}
