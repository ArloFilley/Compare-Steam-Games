/*! 
  FILENAME: models.rs
  AUTHOR:   Arlo Filley
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub response: OwnedGamesResponse,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnedGamesResponse {
    pub game_count: u32,
    pub games: Vec<GameInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameInfo {
    pub appid: u32,
    pub name: String,
    pub img_icon_url: String,
    pub playtime_forever: u32,
    // pub playtime_windows_forever: u32,
    // pub playtime_mac_forever: u32,
    // pub playtime_linux_forever: u32,
    // pub rtime_last_played: i64,
    // pub playtime_disconnected: u32,
}

