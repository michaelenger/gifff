use std::error::Error;
use std::fmt;

use reqwest::{Client, StatusCode, Url};
use serde::{Deserialize, Serialize};

static GFYCAT_CLIENT_ID: &str = "CLIENT ID";
static GFYCAT_CLIENT_SECRET: &str = "CLIENT SECRET";

/// Parameters used when requesting an access code
#[derive(Debug, Serialize)]
struct AccessTokenRequest<'a> {
	client_id: &'a str,
	client_secret: &'a str,
	grant_type: &'a str,
}

/// Contents of an access token response
#[derive(Debug, Deserialize)]
struct AccessTokenReponse {
	access_token: String,
}

/// A single image from Gfycat
#[derive(Debug, Deserialize)]
pub struct GfycatImage {
	#[serde(rename = "gfyId")]
	id: String,
	#[serde(rename = "gifUrl")]
	url: String,
}

/// Contents of a gif response
#[derive(Debug, Deserialize)]
pub struct GifsResponse {
	gfycats: Vec<GfycatImage>
}

/// Error from Gfycat
#[derive(Debug, Deserialize)]
struct GfycatError {
	code: String,
    description: String,
}

impl Error for GfycatError {}
impl fmt::Display for GfycatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.description, self.code)
    }
}

/// Error response
#[derive(Debug, Deserialize)]
struct ErrorResponse {
	#[serde(alias = "errorMessage")]
	error_message: GfycatError,
}

/// Retrieve the access token from Gfycat
fn get_access_token() -> Result<String, Box<dyn Error>> {
	let url = Url::parse("https://api.gfycat.com/v1/oauth/token")?;
	let body = AccessTokenRequest{
		client_id: GFYCAT_CLIENT_ID,
		client_secret: GFYCAT_CLIENT_SECRET,
		grant_type: "client_credentials",
	};

	let client = Client::new();
	let mut response = client.post(url).json(&body).send()?;

    match response.status() {
        StatusCode::OK => {
            let body: AccessTokenReponse = response.json()?;
            Ok(body.access_token)
        }
        _ => {
            let body: ErrorResponse = response.json()?;
            Err(Box::new(body.error_message))
        }
    }
}

/// Search for gifs specified by the query
pub fn search(query: &str) -> Result<Vec<GfycatImage>, Box<dyn Error>> {
	let access_token = get_access_token()?;

    let url = Url::parse_with_params(
        "https://api.gfycat.com/v1/gfycats/search",
        &[
            ("search_text", query),
            ("count", "420"),
        ],
    )?;

    let client = Client::new();
	let mut response = client.get(url).bearer_auth(&access_token).send()?;

	match response.status() {
        StatusCode::OK => {
            let body: GifsResponse = response.json()?;
            Ok(body.gfycats)
        }
        _ => {
            let body: ErrorResponse = response.json()?;
            Err(Box::new(body.error_message))
        }
    }
}

/// Get recent trending gifs
pub fn trending() -> Result<Vec<GfycatImage>, Box<dyn Error>> {
	let access_token = get_access_token()?;

	let url = Url::parse_with_params(
        "https://api.gfycat.com/v1/gfycats/trending",
        &[("count", "420")],
    )?;

    let client = Client::new();
	let mut response = client.get(url).bearer_auth(&access_token).send()?;

	match response.status() {
        StatusCode::OK => {
            let body: GifsResponse = response.json()?;
            Ok(body.gfycats)
        }
        _ => {
            let body: ErrorResponse = response.json()?;
            Err(Box::new(body.error_message))
        }
    }
}
