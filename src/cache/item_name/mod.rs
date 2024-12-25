
use crate::config::config_dir;
use function_name::named;
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use crate::cache::{Cache, CachedData};


impl Cache {
    #[named]
    pub fn try_load_item_names() -> Option<CachedData<HashMap<String, Vec<u32>>>> {
        let path = Self::item_names_file();
        let file = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read item_names_cache: {err}"))
            .ok()?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse item_names_cache: {err}"))
            .ok()?;
        info!(
            "[{}] Loaded item_names_cache from \"{}\"",
            function_name!(),
            path.display()
        );
        Some(config)
    }

    #[named]
    pub fn save_item_names(item_names: &CachedData<HashMap<String, Vec<u32>>>) {
        let path = Self::item_names_file();
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, &item_names)
                    .expect("failed to serialize item_names_cache");
                info!(
                    "[{}] Saved popup_cache to \"{}\"",
                    function_name!(),
                    path.display()
                )
            }
            Err(err) => log::error!("Failed to save item_names_cache: {err}"),
        }
    }

    pub fn item_names_file() -> PathBuf {
        config_dir().join("item_names_cache.json")
    }
    
}
