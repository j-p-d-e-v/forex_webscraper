pub use crate::crawler::headless_browser;
use headless_chrome::{ 
    Tab
};
use serde::{ Serialize, Deserialize };
use std::{
    error::Error, 
    sync::Arc,
};


#[derive(Serialize, Deserialize, Debug)]
pub struct ForexFactoryCalendarNews {
    pub date: String,
    pub dateline: i32,
    pub actual: String,
    pub country: String,
    pub currency: String,
    pub forecast: String,
    pub impact_title: String,
    pub name: String,
    pub previous: String,
    pub time_label: String,
    pub timezone: String,
    pub url: String
}

pub fn get_calendar_news(tab: &Arc<Tab>, calendar: &str) -> Result<Vec<ForexFactoryCalendarNews>, Box<dyn Error>> {
    let url: String = format!("https://www.forexfactory.com/calendar?month={}",calendar);    
    println!("URL: {}",&url);
    tab.navigate_to(&url)?; 
    tab.wait_until_navigated()?; 
    headless_browser::take_screenshot(&tab, format!("cal-{}",calendar).as_str());
    let elem = tab.wait_for_element(".calendar__row")?;
    let remote_obj = elem.call_js_fn(r#"
        function getCalendarStates(){
            let timezone = window.FF.timezone_name;
            let events = window.calendarComponentStates[1]["days"].map( (item)=> item["events"] );
            let joined_events = [];
            for(var i in events){
                for(var e in events[i]){
                    let item = events[i][e];
                    joined_events.push({
                        "actual": item["actual"],
                        "country": item["country"],
                        "currency": item["currency"],
                        "date": item["date"],
                        "dateline": item["dateline"],
                        "forecast": item["forecast"],
                        "impact_title": item["impactTitle"],
                        "name": item["name"],
                        "previous": item["previous"],
                        "url": item["url"],
                        "time_label": item["timeLabel"],
                        "timezone": timezone
                    });
                }
            }            
            return JSON.stringify(joined_events);
        }
    "#, vec![], false)?;
    
    let robj_value = remote_obj.value.unwrap();
    let forex_events_json = robj_value.as_str();
    let forex_events: Vec<ForexFactoryCalendarNews> = serde_json::from_str(forex_events_json.unwrap()).unwrap();
    Ok(forex_events)
}

pub fn run(year: i32) -> Vec<ForexFactoryCalendarNews> {
    let month_names = vec!["jan","feb","mar","apr","may","jun","jul","aug","sep","oct","nov","dec"];
    let mut data: Vec<ForexFactoryCalendarNews> = Vec::new();
    let launch_options = headless_browser::setup();
    for month_name in month_names {
        let browser = headless_browser::browser(&launch_options);
        let tab = headless_browser::create_tab(&browser).unwrap();
        let calendar = format!("{}.{}",month_name,year);
        let mut _data:Vec<ForexFactoryCalendarNews> = get_calendar_news(&tab,calendar.as_str()).unwrap();
        data.append(&mut _data);
    }
    data
}
