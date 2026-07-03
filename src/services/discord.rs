use std::sync::{Arc, OnceLock};

use reqwest::Client;
use twilight_model::{
    channel::message::MessageFlags, http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType}, id::{Id, marker::{ApplicationMarker, InteractionMarker, UserMarker}}, user::User as TwilightUser
};

use crate::{bot::Bot, error::{BotResult, Error}, models::{command::{response::CommandResponse, serializable::SerializableCommand}, user::User}, traits::component::IntoTwilight};



pub (crate) static DISCORD_SERVICE: OnceLock<Arc<DiscordService>> = OnceLock::new();

const BASE_URL: &str = concat!("https://discord.com/api/v", "10");

#[allow(unused)]
pub struct DiscordService {
    client: Arc<Client>
}

#[allow(unused)]
impl DiscordService {
    pub (crate) fn get_or_init(client: Arc<Client>) -> Arc<DiscordService> {
        DISCORD_SERVICE.get_or_init(|| {
            let service = DiscordService::new(client);
            Arc::new(service)
        }).clone()
    }

    pub (crate) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub (crate) async fn update_global_commands(&self, application_id: Id<ApplicationMarker>) -> BotResult<()> {
        let bot = Bot::get_global();
        
        let serializable_commands: Vec<SerializableCommand<'_>> = bot.commands.values()
            .map(|cmd| SerializableCommand(cmd))
            .collect();

        let serialized_commands = serde_json::to_string(&serializable_commands).map_err(|e| Error::JsonFailed(e))?;
        
        let url = format!("{}/applications/{}/commands", BASE_URL, application_id);

        self.client.put(url)
            .body(serialized_commands)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub (crate) async fn defer(&self, id: Id<InteractionMarker>, token: &str, is_edit: bool, ephemeral: bool) -> BotResult<()> {
        let url = format!("{}/interactions/{}/{}/callback", BASE_URL, id, token);

        let kind = if is_edit {
            InteractionResponseType::DeferredUpdateMessage
        } else {
            InteractionResponseType::DeferredChannelMessageWithSource
        };

        let mut payload = CommandResponse::new_with_kind(kind);
        if ephemeral {
            payload.set_ephemeral(ephemeral);
        }

        let response = self.client.post(url)
            .json(&payload.into_twilight())
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            worker::console_error!("API Error [{}]: {}", status, body);
            return Err(Error::Generic(format!("API returned {}: {}", status, body)));
        }

        Ok(())
    }

    pub (crate) async fn edit(&self, application_id: Id<ApplicationMarker>, token: &str, response: CommandResponse) -> BotResult<()> {
        let url = format!("{}/webhooks/{}/{}/messages/@original", BASE_URL, application_id, token);
        let update_payload = response.as_update();

        worker::console_debug!("response_edit_payload: {update_payload:#?}");

        let response = self.client.patch(url)
            .json(&update_payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            worker::console_error!("API Error [{}]: {}", status, body);
            return Err(Error::Generic(format!("API returned {}: {}", status, body)));
        }

        Ok(())
    }

    pub async fn fetch_user(&self, user_id: &Id<UserMarker>) -> BotResult<User> {
        let endpoint = format!("{}/users/{}", BASE_URL, user_id);

        let user: TwilightUser = self.client.get(endpoint)
            .send()
            .await?
            .json()
            .await?;

        Ok(User::from(user))
    }
}