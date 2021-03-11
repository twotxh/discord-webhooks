use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
mod tests;
#[derive(Debug, Serialize, Deserialize)]
pub struct Webhook {
    //r#type: i8,
    id: String,
    name: String,
    token: String,
    avatar: Option<String>,
    channel_id: String,
    guild_id: String,
}
pub struct Client {
    reqwest: reqwest::blocking::Client,
    endpoint: String,
    id: String,
    token: String,
}

impl Webhook {
    // Can't get w/o token since we're not authenticating against Discord Bot API
    pub fn get_webhook(hook: Client) -> Result<Webhook, Error> {
        let client = hook.reqwest;
        let id = hook.id;
        let token = hook.token;
        // Sets url for reqwest to get
        let endpoint: &str =
            &("https://discord.com/api/webhooks/".to_owned() + &id.to_string() + "/" + &token);
        let resp = client.get(endpoint).send()?;
        let respmod = resp.text().unwrap().to_string();
        let webhook: Webhook = serde_json::from_str(&respmod)?;

        Ok(webhook)
    }
    //
    pub fn delete_webhook(hook: Client) -> Result<reqwest::blocking::Response, Error> {
        let client = hook.reqwest;
        let endpoint: &str =
            &("https://discord.com/api/webhooks/".to_owned() + &hook.id + "/" + &hook.token);
        let resp = client.delete(endpoint).send()?;
        if resp.status() != 204 {
            return Err(anyhow!("{}", resp.status()));
        }
        Ok(resp)
    }
    pub fn execute_webhook(
        hookclient: Client,
        body: &'static str,
    ) -> Result<reqwest::blocking::Response, Error> {
        let client = hookclient.reqwest;

        let res = client
            .post(hookclient.endpoint)
            .body(body)
            .header("Content-Type", "application/json")
            .send()?;

        Ok(res)
    }
    pub fn edit_message(
        hookclient: Client,
        body: &'static str,
        message_id: &str,
    ) -> Result<reqwest::blocking::Response, Error> {
        let client = hookclient.reqwest;
        let endpoint: &str = &(hookclient.endpoint + "/messages/" + message_id);
        let res = client
            .patch(endpoint)
            .body(body)
            .header("Content-Type", "application/json")
            .send();
        Ok(res.unwrap())
    }
    pub fn new(id: &str, token: &str) -> Result<Client, Error> {
        Ok(Client {
            id: id.to_string(),
            token: token.to_string(),
            reqwest: reqwest::blocking::Client::new(),
            endpoint: "https://discord.com/api/webhooks/".to_owned()
                + &id.to_string()
                + "/"
                + &token,
        })
    }
}
