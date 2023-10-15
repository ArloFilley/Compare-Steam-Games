/*!
  FILENAME: main.rs
  AUTHOR:   Arlo Filley
*/

#[macro_use] extern crate rocket;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::collections::HashMap;
use std::thread;

mod models;
mod steam_api_types;

use models::Request;

use steam_api_types::{
    User, Game,
    GetOwnedGamesResponse,
    GetPlayerSummariesResponse,
    SharedGames
};

use clap::Parser;

use rocket::State;
use rocket::fs::FileServer;
use rocket::serde::json::Json;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    api_key: String,
}

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request as RocketRequest, Response};

use log::info;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r RocketRequest<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods","POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}


#[launch]
fn rocket() -> _ {
    let args = match Args::try_parse() {
        Err(_) => {
            Args { api_key: std::env::var("API_KEY").unwrap() }
        }
        Ok(args) => {
            args
        }
    };

    rocket::build()
        .attach(CORS)
        .mount("/", routes![get_data])
        .mount("/", FileServer::from("public"))
        .manage(args)
        
}

#[post("/", data ="<request>")]
async fn get_data(request: Json<Request>, args: &State<Args>) -> Json<SharedGames> {
    let now = std::time::Instant::now();

    info!("Start: {:?}", now.elapsed());

    let mut users = vec![];
    let mut u = vec![];

    let mut join_handles = vec![];

    for user in request.steamids.iter() {
        let user = user.clone();
        let api_key = args.api_key.clone();
        join_handles.push(thread::spawn( move || async move {
            let steam_id = match user.parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    match get_id_from_vanity_url(user, &api_key).await {
                        Some(r) => r,
                        None => return None
                    }
                },
            };

            let Some(user_data) = get_owned_games(steam_id, &api_key).await else {
                return None;
            };

            let Some(user_name) = get_user_name(steam_id, &api_key).await else {
                return None;
            };

            Some((user_name, user_data))
        }));
    }

    for handle in join_handles {
        let h = handle.join().unwrap().await;
        let Some((user_name, user_data)) = h else {
            continue;
        };

        users.push(user_data);
        u.push(user_name);
    }

    info!("Data Received: {:?}", now.elapsed());

    let total_users = users.len();
    let mut shared_games: HashMap<u32, Game> = HashMap::new();

    // Loop through all users, 
    for user in users {
        // Loop through all games
        for game in user.response.games {
            // Add playtime or game to the hashmap
            if shared_games.contains_key(&game.appid) {
                shared_games.get_mut(&game.appid).unwrap().playtimes.push(game.playtime_forever);
            } else {
                let appid = game.appid;
                let game = Game { 
                    name: game.name.clone(), 
                    icon: game.img_icon_url.clone(), 
                    playtimes: vec![game.playtime_forever] 
                };
                shared_games.insert(appid, game);
            }
        }
    }
    
    let mut games = vec![];

    for (_, game) in shared_games.drain() {
        let all_users_played = game.playtimes.len() == total_users;
        let playtime_over_filter = game.playtimes.iter().sum::<u32>() >= request.filtertime as u32;
        if all_users_played && playtime_over_filter {
            games.push(game)
        }
    }

    // Game is currently sorted by the total amount of time played by all users
    // This could easily be changed and maybe could be changed depending on 
    // A request parameter
    let sort_function = {
        |a: &Game, b: &Game| 
        b.playtimes.iter().sum::<u32>().cmp(&a.playtimes.iter().sum::<u32>())
    };

    games.sort_by(sort_function);

    let response = SharedGames { users: u, games };
    
    info!("Response: {:?}", now.elapsed());

    Json(response)
}

pub async fn get_user_name(steam_id: u64, api_key: &str) -> Option<User> {
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
        let response: GetPlayerSummariesResponse = serde_json::from_str(&response_text).expect("Couldn't parse");
        let response = response.response.players[0].clone();
        // println!("{response:#?}");
        //
        Some(User { username: response.personaname, avatar: response.avatarfull, user_id: response.steamid.parse().unwrap() } )
    } else {
        eprintln!("Error: Request failed with status code {:?}", response.status());
        None
    }
}

pub async fn get_owned_games(steam_id: u64, api_key: &str) -> Option<GetOwnedGamesResponse> {
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
        let response = serde_json::from_str(&response_text).ok()?;
        
        Some(response)
    } else {
        eprintln!("Error: Request failed with status code {:?}", response.status());
        None
    }
}

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub response: NResponse,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NResponse {
    pub steamid: String,
    pub success: i64,
}


pub async fn get_id_from_vanity_url(vanity_url: String, api_key: &str) -> Option<u64> {
    // basics used for every request
    let base_url    = "api.steampowered.com";
    let api_method  = "ISteamUser/ResolveVanityURL/v0001";

    // The options to be used in the request
    let api_key     = format!("key={}", api_key);
    let vanity_url    = format!("vanityurl={}", vanity_url);
    let format      = "format=json";

    let options     = format!("{api_key}&{vanity_url}&{format}");
    let url         = format!("http://{base_url}/{api_method}/?{options}");

    // Send an HTTP GET request to the URL
    let response = reqwest::get(url).await.ok()?;

    // Check if the request was successful
    if response.status().is_success() {

        // Parse the JSON response into our ApiResponse struct 
        let response_text = response.text().await.ok()?;
        let response = serde_json::from_str::<Root>(&response_text).ok()?;

        if response.response.success == 1 {
            Some(response.response.steamid.parse().unwrap())
        } else {
            None
        }
    } else {
        eprintln!("Error: Request failed with status code {:?}", response.status());
        None
    }
}