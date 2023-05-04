use sled::{Db, Error};
use std::path::PathBuf;
use directories::ProjectDirs;
use bincode;
use log::info;

use types::market::{
    Market,
};

pub fn get_markets() -> Result<Vec<Market>, Error> {
    info!("retrieving markets from db");
    let db = open_db();
    let data = db.get("markets")?;
    match data {
        Some(d) => {
            let markets: Vec<Market> = bincode::deserialize(&d).unwrap();
            Ok(markets)
        },
        None => {
            db.flush()?;
            Ok(vec![])
        }
    }
}

pub fn set_markets(data: Vec<Market>) -> Result<bool, Error> {
    info!("storing markets");
    let db = open_db();
    let serialized_data = bincode::serialize(&data).unwrap();
    db.insert("markets", serialized_data).unwrap();
    db.flush()?;
    Ok(true)
}

fn open_db() -> Db {
    let config_dir = get_config_dir().unwrap();
    let db_path = std::path::PathBuf::from(config_dir).join("data"); 
    sled::open(db_path).unwrap()
}

fn get_config_dir() -> Option<PathBuf> {
    let qualifier = "";
    let org = "";
    let app = "deneb";

    let project_dirs = ProjectDirs::from(qualifier, org, app)
        .expect("failed to get project directories");
    let dir = project_dirs.data_dir();

    Some(dir.to_path_buf())
}