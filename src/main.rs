use chrono;
use reqwest::StatusCode;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

mod utils;

fn save_raw_html(raw_html: &str, domain_name: &str) -> std::io::Result<()> {
    let dt = chrono::Local::now();
    let filename = format!("{}_{}.html", domain_name, dt.format("%Y-%m-%d_%H.%M.%S"));
    let mut writer = File::create(&filename)?;
    write!(&mut writer, "{}", &raw_html)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = utils::get_client();
    let domain_name = "renegade-wing-draft.vercel.app/pov/boilingpointep1";
    let url = format!("https://{}", domain_name);
    let result = client.get(url).send().await?;

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await?,
        _ => panic!("Something went wrong, could not get raw HTML"),
    };

    let document = Html::parse_document(&raw_html);
    let paragraph_selector = Selector::parse("p").unwrap();
    
    for element in document.select(&paragraph_selector) {
        let inner = element.inner_html().to_string();
        println!("{}", &inner);              
    }

    save_raw_html(&raw_html, domain_name)?;
    Ok(())
}



