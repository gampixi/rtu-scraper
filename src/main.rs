use chrono::prelude::*;
use chrono_tz::Europe::Riga;
use stringplus::*;

#[derive(Debug)]
struct Lesson {
    /*subject: String,
    lecturer: String,
    room: String*/
    all_text: String,
    timestamp_start: i64,
    timestamp_end: i64
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let year = 2020;
    let month = 2;
    let url = format!("https://nodarbibas.rtu.lv/grafiks.php?lang=lv&month={}&action=callendar&id=29839&year={}", month, year);
    let resp = reqwest::get(&url)
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
            let day_number = day.select(&day_number_selector).last().unwrap()
                .inner_html().parse::<u32>().unwrap();
            //let time_interval_iter = all_text.chars().take(11);
            //let time_start = time_interval_iter.take(5).collect();
            //println!("{}", all_text);
            //let new_event = Lesson {all_text: event.inner_html()};
            //events.push(new_event);
            let time_start = all_text.as_str().substring(0,5);
            let time_end = all_text.as_str().substring(6,5);
            

            //let time_start = (time_start.substring(0,2).parse::<i32>().unwrap(), time_start.substring(2,2).parse::<i32>().unwrap());
            //let time_end = (time_end.substring(0,2).parse::<i32>().unwrap(), time_end.substring(2,2).parse::<i32>().unwrap());

            let date_start = format!("{} {} {} {}:{}", year, month, day_number, time_start, "00");
            let date_end = format!("{} {} {} {}:{}", year, month, day_number, time_end, "00");

            let date_start = NaiveDateTime::parse_from_str(&date_start, "%Y %m %d %H:%M:%S").unwrap();
            let date_end = NaiveDateTime::parse_from_str(&date_end, "%Y %m %d %H:%M:%S").unwrap();

            let date_start = Riga.from_local_datetime(&date_start).unwrap();
            let date_end = Riga.from_local_datetime(&date_end).unwrap();

            let lesson = Lesson {
                all_text: String::from("farts"),
                timestamp_start: date_start.timestamp(),
                timestamp_end: date_end.timestamp()
            };

            println!("Lesson: {:#?}", lesson);
        }

        /*

        let new_day = Day {day: day_number, lessons: events};

        println!("{:#?}", new_day);*/
    }

    Ok(())
}