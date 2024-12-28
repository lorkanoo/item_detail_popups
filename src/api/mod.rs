use std::time::Duration;

use lazy_static::lazy_static;

pub mod gw2_api;
pub mod gw2_wiki;
pub mod gw2tp;

lazy_static! {
    static ref UREQ_AGENT: ureq::Agent = {
        ureq::AgentBuilder::new()
            .user_agent("ItemPopups/1.0")
            .timeout_connect(Duration::from_secs(3))
            .timeout_read(Duration::from_secs(10))
            .build()
    };
}

#[allow(clippy::result_large_err)]
fn get_sync(url: String) -> Result<ureq::Response, ureq::Error> {
    UREQ_AGENT.get(&url).set("Connection", "keep-alive").call()
}
