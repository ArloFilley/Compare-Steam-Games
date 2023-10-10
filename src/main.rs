/*!
  FILENAME: main.rs
  AUTHOR:   Arlo Filley
*/

#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

mod models;

use crate::models::ApiResponse;
use crate::models::User;
use crate::models::Game;
use crate::models::SharedGames;
use crate::models::UserApiResponse;
use crate::models::Request;

use clap::Parser;

use rocket::fs::FileServer;
use rocket::serde::json::Json;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    api_key: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![get_data])
        .mount("/public", FileServer::from("public"))
}

#[post("/", data ="<request>")]
async fn get_data(request: Json<Request>) -> Json<SharedGames> {
    println!("{request:#?}");
    // Parse command line arguments
    let args = Args::parse();

    let Some(user1) = get_user_data(request.steamids[0].parse::<u64>().unwrap(), &args.api_key).await else {
        panic!("Error getting user data");
    };

    let Some(user2) = get_user_data(request.steamids[1].parse::<u64>().unwrap(), &args.api_key).await else {
        panic!("Error getting user data");
    };

    let player2_games: Vec<String> = user2.response.games.clone().into_iter().map(|a| a.name).collect();

    let mut shared_games: HashMap<u32, Game> = HashMap::new();

    for game in user1.response.games {
        if player2_games.contains(&game.name) {
            shared_games.insert(game.appid,
                Game {
                    name: game.name, 
                    icon: format!("http://media.steampowered.com/steamcommunity/public/images/apps/{}/{}.jpg", game.appid, game.img_icon_url), 
                    playtimes: vec![game.playtime_forever] 
                }
            );
        }
    }

    for game in user2.response.games {
        if shared_games.get(&game.appid).is_some() {
            shared_games.get_mut(&game.appid).unwrap().playtimes.push(game.playtime_forever);
        }
    }

    let mut shared_games: Vec<Game> = shared_games.into_values().collect();
    let total_playtime = |a: &Game, b: &Game| b.playtimes.iter().sum::<u32>().cmp(&a.playtimes.iter().sum());
    shared_games.sort_by(total_playtime);

    let Some(user1) = get_user_name(request.steamids[0].parse::<u64>().unwrap(), &args.api_key).await else {
        panic!();
    };

    let Some(user2) = get_user_name(request.steamids[1].parse::<u64>().unwrap(), &args.api_key).await else {
        panic!();
    };

    let shared_games = shared_games.into_iter().filter(|a| a.playtimes.iter().sum::<u32>() >= request.filtertime as u32).collect::<Vec<Game>>();

    let shared_games = SharedGames {
        users: vec![user1, user2],
        games: shared_games
    };


    // Write to a file for debugging in case of parsing error
    std::fs::write( 
        "output.json",
        serde_json::to_string_pretty(&shared_games).expect("serialization error")
    ).expect("error writing file");

    Json(shared_games)
}

pub async fn get_user_name(steam_id: u64, api_key: &str) -> Option<User> {
    let s = steam_id;

    // basics used for every request
    let base_url    = "api.steampowered.com";
    let api_method  = "ISteamUser/GetPlayerSummaries/v0002";

    // The options to be used in the request
    let api_key     = format!("key={}", api_key);
    let steam_id    = format!("steamids={}", steam_id);

    let options     = format!("{api_key}&{steam_id}");
    let url         = format!("http://{base_url}/{api_method}/?{options}");

    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await.ok()?;

    // Check if the request was successful
    if response.status().is_success() {

        // Parse the JSON response into our ApiResponse struct 
        let response_text = response.text().await.ok()?;
        let response: UserApiResponse = serde_json::from_str(&response_text).expect("Couldn't parse");
        let response = response.response.players[0].clone();
        // println!("{response:#?}");
        //
        Some(User { username: response.personaname, avatar: response.avatarfull, user_id: s } )
    } else {
        eprintln!("Error: Request failed with status code {:?}", response.status());
        None
    }
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
        let response: ApiResponse = serde_json::from_str(&response_text).expect("Couldn't parse");
        
        Some(response)
    } else {
        eprintln!("Error: Request failed with status code {:?}", response.status());
        None
    }
}