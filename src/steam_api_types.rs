/*! 
  FILENAME: models.rs
  AUTHOR:   Arlo Filley
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetOwnedGamesResponse {
    pub response: OwnedGames,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnedGames {
    pub game_count: u32,
    pub games: Vec<GameInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameInfo {
    pub appid: u32,
    pub name: String,
    pub img_icon_url: String,
    pub playtime_forever: u32,
    // pub playtime_windows_forever: Option<u32>,
    // pub playtime_mac_forever: Option<u32>,
    // pub playtime_linux_forever: Option<u32>,
    // pub rtime_last_played: i64,
    // pub playtime_disconnected: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SharedGames {
    pub users: Vec<User>,
    pub games: Vec<Game>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub avatar: String,
    pub user_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub name: String,
    pub icon: String,
    pub playtimes: Vec<u32>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetPlayerSummariesResponse {
    pub response: PlayerSummaries,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerSummaries {
    pub players: Vec<PlayerSummary>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerSummary {
    pub steamid: String,
    // pub communityvisibilitystate: i32,
    // pub profilestate: i32,
    pub personaname: String,
    // pub profileurl: String,
    // pub avatar: String,
    // pub avatarmedium: String,
    pub avatarfull: String,
    // pub avatarhash: String,
    // pub lastlogoff: i64,
    // pub personastate: i32,
    // pub realname: String,
    // pub primaryclanid: String,
    // pub timecreated: i64,
    // pub personastateflags: i32,
}