//! TODO mod docs

use crate::{Capability, Command};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HttpRequest {
    pub method: String, // FIXME this probably should be an enum instead.
    pub url: String,
    // TODO support headers
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,   // FIXME this probably should be a giant enum instead.
    pub body: Vec<u8>, // TODO support headers
}

pub struct Http<Ef> {
    effect: Box<dyn Fn(HttpRequest) -> Ef + Sync>,
}

impl<Ef> Http<Ef> {
    pub fn new<MakeEffect>(effect: MakeEffect) -> Self
    where
        MakeEffect: Fn(HttpRequest) -> Ef + Sync + 'static,
    {
        Self {
            effect: Box::new(effect),
        }
    }

    pub fn get<Ev, F>(&self, url: &str, callback: F) -> Command<Ef, Ev>
    where
        Ev: 'static,
        F: Fn(HttpResponse) -> Ev + Send + Sync + 'static,
    {
        self.request("GET", url, callback)
    }

    pub fn request<Ev, F>(&self, method: &str, url: &str, callback: F) -> Command<Ef, Ev>
    where
        Ev: 'static,
        F: Fn(HttpResponse) -> Ev + Send + Sync + 'static,
    {
        let request = HttpRequest {
            method: method.to_string(),
            url: url.to_string(),
        };

        Command::new((self.effect)(request), callback)
    }
}

impl<Ef> Capability for Http<Ef> where Ef: Clone {}
