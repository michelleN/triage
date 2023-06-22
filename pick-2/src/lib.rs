use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_triage_generator(_req: Request) -> Result<Response> {
    let triage_list = [
        "Person A", "Person B", "Person C", "Person D", "Person E", "Person F", "Person G",
        "Person H", "Person I", "Person J", "Person K", "Person L", "Person M", "Person N",
    ];

    let mut rng = thread_rng();
    let chosen = triage_list.choose_multiple(&mut rng, 2);

    let rotation: Vec<String> = chosen.into_iter().map(|c| c.to_string()).collect();

    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(Some(rotation.join(" and ").into()))?)
}
