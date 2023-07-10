fn main() {
    println!("Hello, world!");
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
