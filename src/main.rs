use dotenv::dotenv;
use std::env;
use sea_orm::{ 
    EntityTrait,
    QueryFilter,
    ColumnTrait,
    DatabaseConnection,
    ActiveValue::{ Set  },
};
pub use forex_webscraper::models::forex_factory_news;
pub use forex_webscraper::crawler::forex_factory::ForexFactoryCalendarNews;
pub use forex_webscraper::crawler::forex_factory;
pub use forex_webscraper::db_connect;

#[tokio::main]
async fn main(){
    dotenv().ok();
    let start_year: i32 = env::var("START_YEAR").expect("START_YEAR in env is required").parse::<i32>().unwrap();
    let end_year: i32 = env::var("END_YEAR").expect("END_YEAR in env is required").parse::<i32>().unwrap();
    let db_conn: DatabaseConnection = db_connect().await.unwrap();
    let db_connected: bool = db_conn.ping().await.is_ok();
    if !db_connected {
        panic!("Not connected to the database.")
    }
    for year in start_year..(end_year + 1) {
        let data: Vec<ForexFactoryCalendarNews> = forex_factory::run(year);
        let mut ff_news: Vec<forex_factory_news::ActiveModel> = Vec::new();
        for item in data {
            ff_news.push(
                forex_factory_news::ActiveModel {
                    id:Set(0),
                    year: Set(year),
                    date: Set(item.date.to_owned()),
                    date_line: Set(item.dateline),
                    actual: Set(item.actual.to_owned()),
                    country: Set(item.country.to_owned()),
                    currency: Set(item.currency.to_owned()),
                    forecast: Set(item.forecast.to_owned()),
                    impact_title: Set(item.impact_title.to_owned()),
                    name: Set(item.name.to_owned()),
                    previous: Set(item.previous.to_owned()),
                    time_label: Set(item.time_label.to_owned()),
                    timezone: Set(item.timezone.to_owned()),
                    url: Set(item.url.to_owned())
                }
            );
        }
        let total_news: usize = ff_news.len();
        
        forex_factory_news::Entity::delete_many().filter(
            forex_factory_news::Column::Year.eq(year)
        ).exec(&db_conn).await.unwrap();
        forex_factory_news::Entity::insert_many(ff_news).exec(&db_conn).await.unwrap();
        println!("INSERTED {} news for year {}",total_news,year);
    }
}