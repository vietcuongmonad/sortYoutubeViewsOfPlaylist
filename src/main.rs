#![allow(non_snake_case)]
#![allow(dead_code)]

use std::env;
use dotenv::dotenv;
use serde_json::Value;

use futures::{stream, StreamExt, TryStreamExt};
use num_format::{Locale, ToFormattedString};

async fn get_video_data(
    playlist_data: &serde_json::Value,
    developer_key: &str,
) -> Result<Vec<(String, u64)>, Box<dyn std::error::Error>> {
    let videos = stream::iter(playlist_data["items"].as_array().unwrap())
        .then(|item| async move {
            let title = item["snippet"]["title"].as_str().unwrap().to_string();
            let video_id = item["snippet"]["resourceId"]["videoId"].as_str().unwrap().to_string();
            let view_count = get_view_count(video_id, developer_key).await?;
            Ok::<_, Box<dyn std::error::Error>>((title, view_count))
        })
        .try_collect()
        .await?;
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let developer_key = env::var("DEVELOPER_KEY").expect("DEVELOPER_KEY must be set");

    let playlist_id = "PL4bD_p5B-nBnHmJCqkdhve5TAoop6I57P";
    let max_vid = 3;

    let url = format!(
        "https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults={}&playlistId={}&key={}", 
        max_vid,
        playlist_id, 
        developer_key
    ); 

    let response = reqwest::get(&url).await?.text().await?;
    let playlist_data: Value = serde_json::from_str(&response)?;

    let videos = get_video_data(&playlist_data, &developer_key).await.unwrap();

    for (title, view_count) in videos {
        println!("{}: {}", title, view_count.to_formatted_string(&Locale::en));
    }

    Ok(())
}

// use reqwest;
// use serde_json::{Value};

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let api_key = "YOUR_API_KEY";
//     let playlist_id = "PL4bD_p5B-nBnHmJCqkdhve5TAoop6I57P";
//     let url = format!("https://www.googleapis.com/youtube/v3/playlistItems?part=snippet&maxResults=50&playlistId={}&key={}", playlist_id, api_key);
    
//     let response = reqwest::get(&url).await?.text().await?;
//     let data: Value = serde_json::from_str(&response)?;
    
//     let video_ids: Vec<String> = data["items"].as_array().unwrap().iter().map(|item| {
//         item["snippet"]["resourceId"]["videoId"].as_str().unwrap().to_string()
//     }).collect();
    
//     println!("{:?}", video_ids);
    
//     Ok(())
// }
