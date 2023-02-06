use std::fs;
use std::sync::Arc;

use headless_chrome::{LaunchOptions, Browser, Tab};
use headless_chrome::protocol::cdp::Page;

fn main() {
    render_hacker_news();
}

fn render_hacker_news() -> Result<String, anyhow::Error> {
    let is_headless = true;
    let is_http = false;
    let target: String = String::from("news.ycombinator.com/");
    
    let options = LaunchOptions::default_builder()
        .headless(is_headless)
        .build()
        .expect("Couldn't find appropriate Chrome binary.");

    let url = &(if is_http { "http://" } else { "https://" }.to_owned() + target.as_str());
    println!("Fetching {}", &url);
    let browser = Browser::new(options)?;
    let tab = &browser.wait_for_initial_tab()?;
    tab.navigate_to(&url).expect("failed to navigate");
    let content = &tab.get_content()?;
    take_screenshot(tab, String::from("some.png"));

    println!("Done. Content size: {}kb", content.len() / 1024);
    Ok(content.to_string())
}

fn take_screenshot(tab: &Arc<Tab>, filename: String) -> () {
    match &tab.wait_for_element("#hnmain") {
        Ok(element) => {
            let screenshot = element.capture_screenshot(Page::CaptureScreenshotFormatOption::Png);
            if let Ok(content) = screenshot.as_ref() {
                fs::write(filename, content);
            }
            ()
        }
        Err(err) => eprintln!("Could'nt capture screenshot {}", err.to_string())
    }
    ()
}
