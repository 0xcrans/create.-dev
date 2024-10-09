use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    pub path: String,
    pub params: Option<HashMap<String, String>>,
    pub query: Option<HashMap<String, Vec<String>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: Base64VecU8,
    },
    Redirect {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    NotFound {
        #[serde(rename = "statusCode")]
        status_code: u32,
    },
}

impl Web4Response {
    pub fn html_response(html: String) -> Self {
        Self::Body {
            content_type: String::from("text/html; charset=UTF-8"),
            body: html.as_bytes().to_owned().into(),
        }
    }

    pub fn plain_response(text: String) -> Self {
        Self::Body {
            content_type: String::from("text/plain; charset=UTF-8"),
            body: text.as_bytes().to_owned().into(),
        }
    }

    pub fn redirect(url: String) -> Self {
        Self::Redirect { body_url: url }
    }

    pub fn not_found() -> Self {
        Self::NotFound { status_code: 404 }
    }
}
