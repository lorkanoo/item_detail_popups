use lazy_static::lazy_static;
use std::time::Duration;

lazy_static! {
    static ref UREQ_AGENT: ureq::Agent = {
        ureq::AgentBuilder::new()
            .user_agent("ItemPopups/1.0")
            .timeout_connect(Duration::from_secs(10))
            .build()
    };
}

#[allow(clippy::result_large_err)]
pub fn get_sync(url: String) -> Result<ureq::Response, ureq::Error> {
    UREQ_AGENT.get(&url).call()
}
