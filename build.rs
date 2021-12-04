use std::{
    error::Error,
    io::{Read, Write},
};

use chrono::{Datelike, FixedOffset};
use flate2::read::GzDecoder;
use hyper::{Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let now = chrono::Utc::now().with_timezone(&FixedOffset::west(5 * 3600));
    for i in 1..=now.day() {
        let file_path = format!("./inputs/day{}_input", i);
        if !std::path::Path::new(&file_path).exists() {
            let input = download_input(2021, i).await;
            if let Ok(input) = input {
                let mut file = std::fs::File::create(file_path)?;
                file.write_all(input.as_bytes())?;
                file.flush()?;
            }
        }
    }
    Ok(())
}

pub async fn download_input(year: i32, day: u32) -> Result<String, Box<dyn Error>> {
    let current_year = chrono::Local::now().year();
    assert!(year > 2010);
    assert!(year <= current_year);
    assert!(day > 0);
    assert!(day <= 31);
    let uri: Uri = format!("https://adventofcode.com/{}/day/{}/input", year, day).parse()?;
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let request = Request::builder().uri(uri)
        // .header("Host", "adventofcode.com")
        // .header("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:94.0) Gecko/20100101 Firefox/94.0")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
        // .header("Accept-Language", "en-US,en;q=0.5")
        .header("Accept-Encoding", "gzip, deflate, br")
        // .header("Referer", "https://adventofcode.com/2020/day/7")
        // .header("Connection", "keep-alive")
        .header("Cookie", "session=53616c7465645f5fc8aec1b93d572a9c52fd0ee71a295f957692ff3c41ffa9d0217068aaa61ddf07b547afcb07121d4b")
        // .header("Upgrade-Insecure-Requests", "1")
        // .header("Sec-Fetch-Dest", "document")
        // .header("Sec-Fetch-Mode", "navigate")
        // .header("Sec-Fetch-Site", "same-origin")
        // .header("Sec-Fetch-User", "?1")
        // .header("Cache-Control", "max-age=0")
        // .header("TE", "trailers")
        .body(Body::empty())?;
    let resp = client.request(request).await?;
    let bytes = hyper::body::to_bytes(resp.into_body())
        .await
        .unwrap()
        .to_vec();
    let mut s = String::new();
    let mut d = GzDecoder::new(bytes.as_slice());
    d.read_to_string(&mut s)?;
    Ok(s)
}
