use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use function_name::named;
use log::info;
use serde::{Deserialize, Serialize};
use crate::cache::{Cache, CachedData};
use crate::config::config_dir;

#[derive(Clone, Serialize, Deserialize)]
pub struct Price {
    highest_buy: u32,
    lowest_sell: u32,
}

impl Cache {
    
    #[named]
    pub fn try_load_prices() -> Option<HashMap<u32, CachedData<Price>>> {
        let path = Self::prices_file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read price_cache: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse price_cache: {err}"))
            .ok()?;
        info!(
                "[{}] Loaded price_cache from \"{}\"",
                function_name!(),
                path.display()
            );
        Some(config)
    }
    
    #[named]
    pub fn save_prices(popups: &HashMap<u32, CachedData<Price>>) {
        let path = Self::prices_file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &popups)
                    .expect("failed to serialize price_cache");
                info!(
                        "[{}] Saved price_cache to \"{}\"",
                        function_name!(),
                        path.display()
                    )
            }
            Err(err) => log::error!("Failed to save price_cache: {err}"),
        }
    }
    
    pub fn prices_file() -> PathBuf {
        config_dir().join("price_cache.json")
    }

}