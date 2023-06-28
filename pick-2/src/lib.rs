use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, thread_rng};
use spin_sdk::{
    config,
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_triage_pair(_req: Request) -> Result<Response> {
    const MAINTAINERS_CONFIG_VARIABLE: &str = "maintainers";

    let maintainers = config::get(MAINTAINERS_CONFIG_VARIABLE)?;
    let maintainers_list: Vec<&str> = maintainers.split(',').map(|s| s.trim()).collect();

    let mut rng = thread_rng();
    let chosen = maintainers_list.choose_multiple(&mut rng, 2);

    let rotation: Vec<String> = chosen.into_iter().map(|c| c.to_string()).collect();

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(Some(rotation.join(" and ").into()))?)
}
