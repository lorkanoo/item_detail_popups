use lazy_static::lazy_static;

pub mod gw2_wiki;
pub mod gw2tp;

lazy_static! {
    static ref UREQ_AGENT: ureq::Agent = {
        ureq::AgentBuilder::new()
            .user_agent("ItemPopups/1.0")
            .build()
    };
}

fn get_sync(url: String) -> Result<ureq::Response, ureq::Error> {
    UREQ_AGENT.get(&url).set("Connection", "keep-alive").call()
}
