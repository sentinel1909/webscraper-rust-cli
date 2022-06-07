use chrono;
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;


mod utils;

fn save_raw_html(raw_html: &str, target: &str) -> std::io::Result<()> {
    let dt = chrono::Local::now();
    let filename = format!("{}_{}.html", target, dt.format("%Y-%m-%d_%H.%M.%S"));
    let mut writer = File::create(&filename)?;
    write!(&mut writer, "{}", &raw_html)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = utils::get_client();
    let domain_name = "renegade-wing-draft.vercel.app";
    let route = "pov";
    let target = "boilingpointep1";
    let url = format!("https://{}/{}/{}", domain_name, route, target);
    let result = client.get(url).send().await?;

    let raw_html = match result.status() {
        StatusCode::OK => result.text().await?,
        _ => panic!("Something went wrong, could not get raw HTML"),
    };

    save_raw_html(&raw_html, target)?;
    Ok(())
}



