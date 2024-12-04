use std::fmt::Display;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::ARKAIVE;

#[derive(Debug)]
pub struct ClassUrl(i32);

impl ClassUrl {
    pub fn id(&self) -> i32 {
        self.0
    }

    pub fn url(&self) -> String {
        format!("{}/courses/{}", ARKAIVE, self.0)
    }

    pub fn new(url: &str) -> Result<Self, crate::Error> {
        let Some(id) = url.split('/').last() else {
            return Err(crate::Error::ParseError);
        };

        let id = id.parse().map_err(|_| crate::Error::ParseError)?;
        Ok(Self(id))
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub url: ClassUrl,
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.url.url())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckinResponse {
    #[serde(rename = "buttonText")]
    pub button_text: String,

    #[serde(rename = "buttonType")]
    pub button_type: String,

    #[serde(rename = "buttonDisabled")]
    pub button_disabled: bool,

    #[serde(rename = "buttonLogout")]
    pub button_logout: Option<String>, // Assuming it's an optional field

    pub status: String,
}

pub async fn list_classes(client: &mut Client) -> Result<Vec<Class>, crate::Error> {
    let res = client
        .get(format!("{}/courses", ARKAIVE))
        .send()
        .await
        .map_err(crate::Error::ConnectionError)?;
    let status = res.status();

    if status == reqwest::StatusCode::FOUND {
        return Err(crate::Error::NotAuthenticated);
    }

    if status != reqwest::StatusCode::OK {
        return Err(crate::Error::UnexpectedStatusCode(status));
    }

    let body = res.text().await.map_err(crate::Error::ConnectionError)?;
    let document = scraper::Html::parse_document(&body);

    let class =
        scraper::Selector::parse("a.list-group-item").map_err(|_| crate::Error::ParseError)?;
    let class_title = scraper::Selector::parse("h4 b").map_err(|_| crate::Error::ParseError)?;

    document
        .select(&class)
        .map(|element| {
            let url = element.value().attr("href").unwrap();
            let url = ClassUrl::new(url)?;
            let name = element
                .select(&class_title)
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            Ok(Class { name, url })
        })
        .collect()
}

pub async fn checkin(class: i32, client: &mut Client) -> Result<CheckinResponse, crate::Error> {
    let params = [("id", class)];
    let res = client
        .post(format!("{}/checkin/ajax", ARKAIVE))
        .form(&params)
        .send()
        .await
        .map_err(crate::Error::ConnectionError)?;
    let status = res.status();

    if status == reqwest::StatusCode::FOUND {
        return Err(crate::Error::NotAuthenticated);
    }

    if status != reqwest::StatusCode::OK {
        return Err(crate::Error::UnexpectedStatusCode(status));
    }

    res.json()
        .await
        .map_err(crate::Error::ConnectionError)
        .map_err(|_| crate::Error::ParseError)
}
