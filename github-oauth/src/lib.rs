use anyhow::{anyhow, bail, Result};
use cookie::Cookie;
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

mod api {
    use super::*;

    pub fn handle_github_auth(req: Request, _params: Params) -> anyhow::Result<Response> {
        let Some(url) = req
            .headers()
            .get("spin-full-url") 
            .and_then(|url| url.to_str().ok())
            .and_then(|url| Url::parse(url).ok()) else {
                return http_error(http::StatusCode::INTERNAL_SERVER_ERROR, None)
            };

        let Some(code_param) = get_query_param(url.clone(), "code") else {
            return http_error(http::StatusCode::BAD_REQUEST, None);
        };

        let Some(state_param) = get_query_param(url, "state") else {
            return http_error(http::StatusCode::BAD_REQUEST, None);
        };

        let Some(host) = req.headers().get("host").and_then(|h| h.to_str().ok()) else {
            return http_error(http::StatusCode::INTERNAL_SERVER_ERROR, None)
        };

        let Some(stored_state) = get_state_from_cookie(&req.headers()) else {
                return http_error(http::StatusCode::UNAUTHORIZED, None);
        };

        if state_param != stored_state {
            return http_error(http::StatusCode::UNAUTHORIZED, None);
        }

        let mut redirect = format!("https://{}/api/sessions/oauth/github", host);
        if host.contains("localhost") || host.contains("127.0.0.1") {
            redirect = format!("http://{}/api/sessions/oauth/github", host);
        }

        let token = match exchange_code_for_token(&code_param, &redirect) {
            Ok(t) => t,
            Err(e) => {
                return http_error(http::StatusCode::UNAUTHORIZED, None);
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

        let Ok(maintainers) =  maintainers() else {
                return http_error(http::StatusCode::INTERNAL_SERVER_ERROR, None)
        };

        if maintainers
            .iter()
            .find(|&m| m.to_owned() == username)
            .is_none()
        {
            return http_error(http::StatusCode::FORBIDDEN, None);
        }

        let cookie_value = format!("oauth_token={}; Secure; HttpOnly", token);
        let login_cookie = "login=success; Secure;";

        Ok(http::Response::builder()
            .status(302)
            .header(http::header::CONTENT_TYPE, "text/plain")
            .header(http::header::LOCATION, "http://localhost:3000") // TODO use spin headers
            .header(http::header::SET_COOKIE, &cookie_value)
            .header(http::header::SET_COOKIE, login_cookie)
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

fn get_state_from_cookie(headers: &http::HeaderMap<http::HeaderValue>) -> Option<String> {
    let Some(cookie_header) = headers
        .get(http::header::COOKIE)
        .and_then(|h| h.to_str().ok()) else {
            return None;
        };

    for c in Cookie::split_parse(cookie_header) {
        match c {
            Ok(c) => {
                if c.name() == "state" {
                    return Some(c.value().to_owned());
                }
            }
            _ => {
                continue;
            }
        }
    }

    return None;
}

fn http_error(status: http::StatusCode, message: Option<&str>) -> Result<Response> {
    let message = message.unwrap_or("error");
    Ok(http::Response::builder()
        .status(status)
        .body(Some(message.to_owned().into()))?)
}

fn maintainers() -> Result<Vec<String>> {
    let maintainers = config::get(MAINTAINERS_CONFIG_VARIABLE)?;
    Ok(maintainers
        .split(',')
        .map(|s| s.trim().to_owned())
        .collect())
}
