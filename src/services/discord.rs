
use std::{sync::{Arc, OnceLock}, time::Duration};

use reqwest::{Method, RequestBuilder};
use serde_json::json;
use twilight_model::{
    channel::{Channel, ChannelType, Message}, guild::Permissions, http::{interaction::InteractionResponseType, permission_overwrite::{PermissionOverwrite, PermissionOverwriteType}}, id::{
        Id, marker::{
            ApplicationMarker, ChannelMarker, GenericMarker, GuildMarker, InteractionMarker, UserMarker
        }
    }, user::User as TwilightUser
};
use worker::Date;

use crate::{bot::{Bot, HTTP_CLIENT}, error::{BotResult, Error}, models::{command::{response::CommandResponse, serializable::SerializableCommand}, message::DiscordMessagePayload, user::User}, traits::component::IntoTwilight};

pub (crate) static DISCORD_SERVICE: OnceLock<Arc<DiscordService>> = OnceLock::new();
const BASE_URL: &str = concat!("https://discord.com/api/v", "10");

pub struct DiscordService {
    token: String
}

impl DiscordService {
    pub (crate) fn get_or_init(token: String) -> Arc<DiscordService> {
        DISCORD_SERVICE.get_or_init(|| Arc::new(Self::new(token))).clone()
    }

    pub (crate) fn new(token: String) -> Self {
        Self {
            token: token
        }
    }

    pub (crate) fn request(&self, method: Method, url: String) -> RequestBuilder {
        // TODO: capire il funzionamento di HTTP_CLIENT.build_split() potrebbe essere utile
        HTTP_CLIENT.request(method, url)
            .header("Authorization", format!("Bot {}", self.token))
            .header("Content-Type", "application/json")
            .timeout(Duration::new(10, 0))
    }

    pub (crate) async fn update_global_commands(&self, application_id: Id<ApplicationMarker>) -> BotResult<()> {
        let bot = Bot::get_global();
        
        let serializable_commands: Vec<SerializableCommand<'_>> = bot.commands.values()
            .map(|cmd| SerializableCommand(cmd))
            .collect();

        let serialized_commands = serde_json::to_string(&serializable_commands).map_err(|e| Error::JsonFailed(e))?;
        
        let url = format!("{}/applications/{}/commands", BASE_URL, application_id);
        self.request(Method::PUT, url)
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

        self.request(Method::POST, url)
            .json(&payload.into_twilight())
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub (crate) async fn edit(&self, application_id: Id<ApplicationMarker>, token: &str, response: CommandResponse) -> BotResult<()> {
        let url = format!("{}/webhooks/{}/{}/messages/@original?with_components=true", BASE_URL, application_id, token);
        let update_payload = response.as_update();

        worker::console_debug!("response_edit_payload: {update_payload:#?}");

        self.request(Method::PATCH, url)
            .json(&update_payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    #[allow(unused)]
    pub (crate) async fn update(&self, interaction_id: Id<InteractionMarker>, interaction_token: &str, response: CommandResponse) -> BotResult<()> {
        let url = format!("{BASE_URL}/interactions/{interaction_id}/{interaction_token}/callback");
        let update_payload = response.as_update();
        
        worker::console_debug!("response_edit_payload: {update_payload:#?}");

        self.request(Method::PATCH, url)
            .json(&update_payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    // Fetch

    pub async fn fetch_bot_user(&self) -> BotResult<User> {
        let endpoint = format!("{}/users/@me", BASE_URL);

        let bot_user: TwilightUser = self.request(Method::GET, endpoint)
            .send()
            .await?
            .json()
            .await?;

        Ok(User::from(bot_user))
    }

    pub async fn fetch_user(&self, user_id: &Id<UserMarker>) -> BotResult<User> {
        let endpoint = format!("{}/users/{}", BASE_URL, user_id);

        let user: TwilightUser = self.request(Method::GET, endpoint)
            .send()
            .await?
            .json()
            .await?;

        Ok(User::from(user))
    }

    pub async fn fetch_messages(&self, channel_id: Id<ChannelMarker>, amount: u8) -> BotResult<Vec<Message>> {
        let endpoint = format!("{}/channels/{}/messages?limit={}", BASE_URL, channel_id, amount);
        let messages: Vec<Message> = self.request(Method::GET, endpoint)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(messages)
    }

    pub async fn fetch_channel(&self, channel_id: Id<ChannelMarker>) -> BotResult<Channel> {
        let endpoint = format!("{}/channels/{}", BASE_URL, channel_id);

        let channel: Channel = self.request(Method::GET, endpoint)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(channel)
    }

    // Delete

    pub async fn delete_messages(
        &self, 
        channel_id: Id<ChannelMarker>, 
        amount: u8
    ) -> BotResult<usize> {
        if amount < 1 || amount > 100 {
            return Err(Error::Generic("The delete takes between 1 and 100 messages.".into()));
        }

        let messages = self.fetch_messages(channel_id, amount).await?;

        if messages.is_empty() {
            return Ok(0);
        }

        const TWO_WEEKS_MS: u64 = 14 * 24 * 60 * 60 * 1000;
        let now_ms = Date::now().as_millis();
        let message_ids: Vec<String> = messages
            .into_iter()
            .filter(|msg| {
                let msg_time_ms = msg.timestamp.as_secs() * 1000;
                let age_ms = now_ms.saturating_sub(msg_time_ms as u64);
                age_ms < TWO_WEEKS_MS
            })
            .map(|msg| msg.id.get().to_string())
            .collect();

        if message_ids.is_empty() {
            return Err(Error::Generic("I messaggi trovati sono troppo vecchi (>14 giorni) o insufficienti per il bulk delete.".into()));
        }

        if message_ids.len() == 1 {
            let endpoint = format!("{}/channels/{}/messages/{}", BASE_URL, channel_id, &message_ids[0]);
            self.request(Method::DELETE, endpoint)
                .send()
                .await?
                .error_for_status()?;
        } else {
            let endpoint = format!("{}/channels/{}/messages/bulk-delete", BASE_URL, channel_id);
            let payload = json!({ "messages": message_ids });

            self.request(Method::POST, endpoint)
                .json(&payload)
                .send()
                .await?
                .error_for_status()?;
        }

        Ok(message_ids.len())
    }

    pub async fn delete_channel(
        &self, 
        channel_id: Id<ChannelMarker>
    ) -> Result<(), Error> {
        let endpoint = format!("{}/channels/{}", BASE_URL, channel_id);

        self.request(Method::DELETE, endpoint)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    // Create

    pub async fn create_channel(
        &self, 
        guild_id: Id<GuildMarker>, 
        name: String, 
        kind: ChannelType, 
        parent_id: Option<Id<GenericMarker>>, 
        position: Option<u16>
    ) -> Result<Channel, Error> {
        let endpoint = format!("{}/guilds/{}/channels", BASE_URL, guild_id);

        let payload = json!({
            "name": name,
            "type": kind,
            "parent_id": parent_id.map(|id| id.get().to_string()),
            "position": position
        });

        let response: Channel = self.request(Method::GET, endpoint)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }

    // Send 

    pub async fn send_message(
        &self, 
        channel_id: Id<ChannelMarker>, 
        payload: &DiscordMessagePayload
    ) -> BotResult<()> {
        let endpoint = format!("{}/channels/{}/messages", BASE_URL, channel_id);

        self.request(Method::POST, endpoint)
            .json(payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn move_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        channel_id: Id<ChannelMarker>
    ) -> BotResult<()> {
        let endpoint = format!(
            "{}/guilds/{}/members/{}", 
            BASE_URL, 
            guild_id, 
            user_id
        );

        let payload = json!({
            "channel_id": Some(channel_id.get().to_string()),
        });

        self.request(Method::PATCH, endpoint)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    pub async fn set_user_permissions(
        &self, 
        channel_id: Id<ChannelMarker>, 
        user_id: Id<UserMarker>, 
        allow: Option<Permissions>, 
        deny: Option<Permissions>,
        kind: PermissionOverwriteType
    ) -> BotResult<()> {
        let endpoint = format!("{}/channels/{}/permissions/{}",BASE_URL, channel_id, user_id);

        let payload = PermissionOverwrite {
            allow, 
            deny,
            id: user_id.cast(),
            kind
        };

        self.request(Method::PUT, endpoint)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}