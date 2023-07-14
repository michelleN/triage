use anyhow::{anyhow, bail, Result};
use spin_sdk::{
    config,
    http::{Params, Request, Response, Router},
    http_component,
};
use url::Url;

const MAINTAINERS_CONFIG_VARIABLE: &str = "maintainers";
const GITHUB_CLIENT_ID_VARIABLE: &str = "id";
const GITHUB_CLIENT_SECRET_VARIABLE: &str = "secret";

// TODO: needs some major refactoring:
// - get rid of router, don't need it for one route
// - general cleanup
// - long term: should be a component that handles github oauth.
//   http handler should import an instance of the "auth interface"

/// A Spin HTTP component that handles github oauth
#[http_component]
fn handle_route(req: Request) -> Result<Response> {
    let mut router = Router::new();
    router.get("/api/sessions/oauth/github", api::handle_github_auth);
    router.handle(req)
}

fn http_error(status: http::StatusCode, message: &str) -> Result<Response> {
    Ok(http::Response::builder()
        .status(status)
        .body(Some(message.to_owned().into()))?)
}

mod api {

    use super::*;

    pub fn handle_github_auth(req: Request, _params: Params) -> anyhow::Result<Response> {
        let maintainers =
            config::get(MAINTAINERS_CONFIG_VARIABLE).expect("unable to parse maintainers config");
        let maintainers_list: Vec<&str> = maintainers.split(',').map(|s| s.trim()).collect();

        let code_str = match req
            .headers()
            .get("spin-full-url")
            .and_then(|url| url.to_str().ok())
            .and_then(|u| Url::parse(u).ok())
            .and_then(|u| get_query_param(u, "code"))
        {
            Some(code) => code,
            None => return http_error(http::StatusCode::BAD_REQUEST, "Could not parse code"),
        };

        let Some(host) = req.headers().get("host").and_then(|h| h.to_str().ok()) else {
            return http_error(http::StatusCode::INTERNAL_SERVER_ERROR, "Could not parse host header")
        };

        let mut redirect = format!("https://{}/api/sessions/oauth/github", host);
        if host.contains("localhost") || host.contains("127.0.0.1") {
            redirect = format!("http://{}/api/sessions/oauth/github", host);
        }

        let token = match exchange_code_for_token(&code_str, &redirect) {
            Ok(t) => t,
            Err(e) => {
                return http_error(
                    http::StatusCode::UNAUTHORIZED,
                    format!("Could not exchange code for token {}", e).as_ref(),
                );
            }
        };

        let username = match get_username(&token) {
            Ok(username) => username,
            Err(e) => {
                return Ok(http::Response::builder()
                    .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Some(format!("Error getting username: {:?}", e).into()))
                    .unwrap());
            }
        };

        if maintainers_list.iter().find(|&&m| m == username).is_none() {
            return http_error(http::StatusCode::FORBIDDEN, "User not in maintainers list");
        }

        let cookie_value = format!("oauth_token={}; Secure; SameSite=Lax", token);
        Ok(http::Response::builder()
            .status(200)
            .header("Content-Type", "text/plain")
            .header("token", token.clone())
            .header(http::header::SET_COOKIE, &cookie_value)
            .body(Some(format!("Hello {}!", username).into()))?)
    }
}

fn get_username(token: &str) -> Result<String> {
    let auth = format!("Bearer {token}");

    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("GET")
            .uri("https://api.github.com/user")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("Authorization", auth)
            .header("User-Agent", "spin-triage")
            .body(None)?,
    )?;

    if !res.status().is_success() {
        bail!("Error getting username: {:?}", res.status().as_u16());
    } else {
        match res.body() {
            Some(r) => {
                // TODO make error messages better
                let login = serde_json::from_slice::<serde_json::Value>(&r)?
                    .get("login")
                    .ok_or(anyhow!("No login found"))?
                    .as_str()
                    .ok_or(anyhow!("Failed to convert login name to string"))? // TODO: error type
                    .to_owned();

                Ok(login)
            }
            None => Err(anyhow!("No body found")), // TODO
        }
    }
}

fn get_query_param(url: Url, param: &str) -> Option<String> {
    let val = url
        .query_pairs()
        .find(|(key, _)| key == param)
        .map(|(_, value)| value.into_owned());

    val
}

// exchange_code_for_token performs the code-to-token exchange with GitHub's
//  OAuth endpoint using the given code and redirect_url.
fn exchange_code_for_token(code: &str, redirect_url: &str) -> Result<String> {
    let client_id = match config::get(GITHUB_CLIENT_ID_VARIABLE) {
        Ok(client_id) => client_id,
        Err(e) => {
            bail!("unable to parse client_id {}", e)
        }
    };

    let client_secret = match config::get(GITHUB_CLIENT_SECRET_VARIABLE) {
        Ok(client_secret) => client_secret,
        Err(e) => bail!("unable to parse client_secret {}", e),
    };

    let b = format!(
        "client_id={client_id}&client_secret={client_secret}&code={code}&redirect_uri={redirect_url}",
    );

    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("POST")
            .uri("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(Some(b.into()))?,
    )?;

    if !res.status().is_success() {
        bail!(
            "Error getting access token: {:?} {:?}",
            res.status().as_u16(),
            res.body()
        );
    } else {
        let Some(body) = res.body().clone() else {
            bail!("No body found");
        };
        let access_token = serde_json::from_slice::<serde_json::Value>(&body)?
            .get("access_token")
            .ok_or(anyhow!("No access token found"))?
            .as_str()
            .ok_or(anyhow!("Failed to convert access token to string"))?
            .to_owned();

        Ok(access_token)
    }
}
