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
pub struct AccessTokenReponse {
	access_token: String,
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

/// Retrieve a result from Gfycat
pub fn get_access_token() -> Result<AccessTokenReponse, Box<dyn Error>> {
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
            Ok(body)
        }
        _ => {
            let body: ErrorResponse = response.json()?;
            Err(Box::new(body.error_message))
        }
    }
}
