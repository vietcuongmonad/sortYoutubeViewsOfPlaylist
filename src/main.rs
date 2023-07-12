#![allow(non_snake_case)]
#![allow(dead_code)]

use std::env;
use dotenv::dotenv;
use serde_json::Value;

use num_format::{Locale, ToFormattedString};

async fn get_video_data(
    playlist_data: &serde_json::Value,
    developer_key: &str,
) -> Result<Vec<(String, u64, String, String)>, Box<dyn std::error::Error>> {
    let items = playlist_data["items"].as_array().unwrap();

    let mut videos = Vec::new();

    for item in items {
        let title = item["snippet"]["title"].as_str().unwrap().to_string();
        let video_id = item["snippet"]["resourceId"]["videoId"].as_str().unwrap().to_string();
        let view_count = get_view_count(video_id.clone(), developer_key).await?;
        let thumbnail = item["snippet"]["thumbnails"]["default"]["url"].as_str().unwrap_or("").to_string();
        let video_url = format!("https://www.youtube.com/watch?v={}", video_id);

        videos.push((title, view_count, thumbnail, video_url));
    }

    videos.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(videos)
}


async fn get_view_count(video_id: String, developer_key: &str) -> Result<u64, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&part=statistics&key={}",
        video_id,
        developer_key
    );
    let response = reqwest::get(&url).await?.text().await?;
    let video_data: Value = serde_json::from_str(&response)?;

    // print_pretty_json(&video_data).await?;

    let view_count = video_data["items"][0]["statistics"]["viewCount"]
        .as_str()
        .unwrap_or("0")
        .parse()
        .unwrap();

    Ok(view_count)
}

async fn print_pretty_json(data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}

fn display_vid(videos: Vec<(String, u64, String, String)>, mut max_display_vid: u64) {
    for (title, view_count, thumbnail, video_url) in videos {
        println!("### {} \n### view count: {}\n", title, view_count.to_formatted_string(&Locale::en));
        println!("[![thumbnail]({})]({})\n", thumbnail, video_url);

        max_display_vid -= 1;
        if max_display_vid == 0 {
            break;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playlist_id = "PL4bD_p5B-nBnHmJCqkdhve5TAoop6I57P";
    let max_vid = 20;

    dotenv().ok();
    let developer_key = env::var("DEVELOPER_KEY").expect("DEVELOPER_KEY must be set");

    let url = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults={}&playlistId={}&key={}", 
        max_vid,
        playlist_id, 
        developer_key
    ); 

    let response = reqwest::get(&url).await?.text().await?;
    let playlist_data: Value = serde_json::from_str(&response)?;

    let videos = get_video_data(&playlist_data, &developer_key).await.unwrap();
    
    display_vid(videos, max_vid);

    Ok(())
}
