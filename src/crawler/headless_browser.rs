use dotenv::dotenv;
use headless_chrome::{ 
    Browser, 
    LaunchOptions,
    Tab,  
    browser::{ 
        LaunchOptionsBuilder
    },
    protocol::cdp::{
        Page, 
        Target::CreateTarget 
    } 
};
use std::{
    fs,
    env,
    error::Error, 
    time::Duration, 
    path::PathBuf,
    collections::HashMap,
    sync::Arc,
    option::Option::Some
};

pub fn setup() -> LaunchOptionsBuilder<'static>{    
    dotenv().ok();
    let current_dir = env::current_dir().unwrap().display().to_string();
    let user_data_dir = PathBuf::from(format!("{}/user-data",current_dir).as_str());

    let mut launch_options: LaunchOptionsBuilder = LaunchOptions::default_builder();
    launch_options.user_data_dir(Some(user_data_dir));
    launch_options
}
pub fn browser(launch_options: &LaunchOptionsBuilder) -> Browser {
    let browser: Browser = Browser::new(launch_options.build().unwrap()).unwrap();
    browser
}

pub fn take_screenshot(tab: &Arc<Tab>, name: &str) {
    
    let data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true).unwrap();
    
    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/screenshots/forexfactory-{}.jpeg",current_dir,name);
    fs::write(&path, data).unwrap();

}

pub fn create_tab(browser: &Browser) -> Result<Arc<Tab>, Box<dyn Error>> {
    let mut headers: HashMap<&str, &str> = HashMap::new();
    headers.insert("user-agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36");
    headers.insert("cookie","fftimeformat=1;fftimezone=Etc/UTC");
    let tab_options = CreateTarget {
        url: String::from("about:blank"),
        width: Some(1024),
        height: Some(700),
        background: None,
        new_window: Some(true),
        browser_context_id: None,
        enable_begin_frame_control: None,
    };
    let tab = browser.new_tab_with_options(tab_options)?;
    tab.set_default_timeout(Duration::from_secs(30));
    tab.set_extra_http_headers(headers)?;
    Ok(tab)
}