/*! 
  FILENAME: models.rs
  AUTHOR:   Arlo Filley
*/

use serde::{ Deserialize, Serialize };

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Request {
    pub steamids: Vec<String>,
    pub filtertime: u64
}
