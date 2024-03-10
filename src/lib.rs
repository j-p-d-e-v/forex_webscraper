pub mod crawler;
pub mod models;
use dotenv::dotenv;
use std::{
    env
};
use sea_orm::{ 
    Database, 
    ConnectOptions, 
    DatabaseConnection,
    error::{ DbErr },
};

pub async fn db_connect() -> Result<DatabaseConnection,DbErr>  {

    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL").expect("Database url must be set.");
    let db_conn_opt = ConnectOptions::new(&db_url);
    //db_conn_opt.set_schema_search_path(db_name);

    let db = Database::connect(db_conn_opt).await?;
    Ok(db)
}