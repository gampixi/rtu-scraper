use chrono::prelude::*;
use stringplus::*;

#[derive(Debug)]
struct Lesson {
    /*subject: String,
    lecturer: String,
    room: String*/
    all_text: String,
    time_start: i64,
    time_end: i64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://nodarbibas.rtu.lv/grafiks.php?action=callendar&id=29839&lang=lv&int=0")
        .await?
        .text()
        .await?;

    let doc = scraper::Html::parse_document(&resp);
    let calendar_day_selector = scraper::selector::Selector::parse("td.calendar-day").unwrap();
    let event_selector = scraper::selector::Selector::parse("div.event").unwrap();
    let day_number_selector = scraper::selector::Selector::parse("div.day-number").unwrap();
    //let mut days: Vec<Day> = vec!();

    for day in doc.select(&calendar_day_selector) {
        let mut events: Vec<Lesson> = vec!();

        for event in day.select(&event_selector) {
            let all_text = event.inner_html();
            //let time_interval_iter = all_text.chars().take(11);
            //let time_start = time_interval_iter.take(5).collect();
            println!("{}", all_text);
            //let new_event = Lesson {all_text: event.inner_html()};
            //events.push(new_event);
            let time_start = all_text.as_str().substring(0,5);
            let time_end = all_text.as_str().substring(6,5);
            println!("Sakums: {} Beigas: {}", time_start, time_end);
        }

        /*let day_number = day.select(&day_number_selector).last().unwrap()
        .inner_html().parse::<u32>().unwrap();

        let new_day = Day {day: day_number, lessons: events};

        println!("{:#?}", new_day);*/
    }

    Ok(())
}