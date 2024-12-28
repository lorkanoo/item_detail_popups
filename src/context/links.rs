use nexus::data_link::mumble::MumblePtr;
use nexus::data_link::{get_mumble_link, get_nexus_link, NexusLink};

#[derive(Debug, Clone)]
pub struct Links {
    pub mumble: Option<MumblePtr>,
    nexus: *const NexusLink,
}

impl Default for Links {
    fn default() -> Self {
        let nexus = get_nexus_link();
        if nexus.is_null() {
            log::error!("Failed to get Nexus link")
        }
        Self {
            mumble: get_mumble_link(),
            nexus,
        }
    }
}

impl Links {
    pub unsafe fn nexus(&self) -> Option<&NexusLink> {
        self.nexus.as_ref()
    }
}

unsafe impl Send for Links {}
unsafe impl Sync for Links {}
