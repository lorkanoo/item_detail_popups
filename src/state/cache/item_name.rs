use crate::configuration::config::config_dir;
use crate::state::cache::cached_data::CachedData;
use log::info;
use log::warn;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

use crate::state::cache::cache::Persist;
use crate::state::cache::cache::StoreInCache;

pub type ItemNamesCache = HashMap<String, Vec<u32>>;

impl Persist for CachedData<ItemNamesCache> {
    fn load(&mut self) {
        let path = CachedData::<ItemNamesCache>::file_path();
        let file_opt = File::open(&path)
            .inspect_err(|err| log::warn!("Failed to read item_names_cache: {err}"))
            .ok();

        if file_opt.is_none() {
            warn!(
                "[load] Failed to load item names cache from \"{}\"",
                path.display()
            );
            return;
        }
        let file = file_opt.unwrap();
        let reader = BufReader::new(file);
        let config_opt = serde_json::from_reader(reader)
            .inspect_err(|err| log::warn!("Failed to parse item_names_cache: {err}"))
            .ok();
        if let Some(config) = config_opt {
            *self = config;
        }
        info!("[load] Loaded item_names_cache from \"{}\"", path.display());
    }

    fn file_path() -> PathBuf {
        config_dir().join("item_names_cache.json")
    }

    fn save(&self) {
        let path = CachedData::<ItemNamesCache>::file_path();
        info!(
            "[save] Attempting to save item_names_cache to \"{}\"",
            path.display()
        );
        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                serde_json::to_writer_pretty(writer, self)
                    .expect("failed to serialize item_names_cache");
                info!(
                    "[save_item_names] Saved popup_cache to \"{}\"",
                    path.display()
                )
            }
            Err(err) => log::error!("Failed to save item_names_cache: {err}"),
        }
    }
}

impl<'a> StoreInCache<'a, CachedData<ItemNamesCache>, &'a ItemNamesCache>
    for CachedData<ItemNamesCache>
{
    fn retrieve(&'a mut self, _key: ()) -> Option<&'a ItemNamesCache> {
        self.value()
    }
}
