/*!
  FILENAME: main.rs
  AUTHOR:   Arlo Filley
*/

extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};
use reqwest::Error;

mod models;

use crate::models::ApiResponse;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    steam_id1: u64,

    #[arg(long)]
    steam_id2: u64,

    #[arg(long)]
    api_key: String,

    #[arg(long)]
    filter: u32,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Parse command line arguments
    let args = Args::parse();

    let Some(mut data1) = get_user_data(args.steam_id1, &args.api_key).await else {
        panic!("Error getting user data");
    };

    let Some(mut data2) = get_user_data(args.steam_id2, &args.api_key).await else {
        panic!("Error getting user data");
    };

    // Write to a file for debugging in case of parsing error
    std::fs::write( 
        format!("{}.json", args.steam_id1), 
        serde_json::to_string_pretty(&data1).expect("serialization error")
    ).expect("error writing file");

    println!(" -> User {} <-", args.steam_id1);
    data1.response.games.sort_by(|a, b| b.playtime_forever.cmp(&a.playtime_forever));
    for game in &data1.response.games {
        if game.playtime_forever >= args.filter {
            println!("{}: {}mins", game.name, game.playtime_forever);
        }
    }

    println!("\n -> User {} <-", args.steam_id2);
    data2.response.games.sort_by(|a, b| b.playtime_forever.cmp(&a.playtime_forever));
    for game in &data2.response.games {
        if game.playtime_forever >= args.filter {
            println!("{}: {}mins", game.name, game.playtime_forever);
        }
    }

    Ok(())
}


pub async fn get_user_data(steam_id: u64, api_key: &str) -> Option<ApiResponse> {
    // basics used for every request
    let base_url    = "api.steampowered.com";
    let api_method  = "IPlayerService/GetOwnedGames/v0001";

    // The options to be used in the request
    let api_key     = format!("key={}", api_key);
    let steam_id    = format!("steamid={}", steam_id);
    let free_games  = "include_played_free_games=true";
    let app_info    = "include_appinfo=true";
    let format      = "format=json";

    let options     = format!("{api_key}&{steam_id}&{free_games}&{app_info}&{format}");
    let url         = format!("http://{base_url}/{api_method}/?{options}");

    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await.ok()?;

    // Check if the request was successful
    if response.status().is_success() {

        // Parse the JSON response into our ApiResponse struct 
        let response_text = response.text().await.ok()?;
        let mut response: ApiResponse = serde_json::from_str(&response_text).expect("Couldn't parse");
        // println!("{response:#?}");
        
        Some(response)
    } else {
        eprintln!("Error: Request failed with status code {:?}", response.status());
        None
    }
}
