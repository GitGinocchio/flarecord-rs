use twilight_model::{
    application::{
        interaction::{
            InteractionContextType, 
            InteractionData, 
            InteractionPartialGuild, 
            application_command::{
                CommandData as TwilightCommandData, 
                CommandOptionValue as TwilightCommandOptionValue,
            }
        }, 
        monetization::Entitlement
    }, 
    channel::{Channel, Message}, 
    guild::{
        PartialMember, 
        Permissions
    }, 
    id::{
        AnonymizableId, 
        Id,
        marker::{
            ApplicationMarker, 
            ChannelMarker, 
            GuildMarker, 
            InteractionMarker, 
            UserMarker
        }
    }, 
    oauth::ApplicationIntegrationMap
};

use crate::{error::{BotResult, Error}, models::{command::{data::CommandData, response::CommandResponse}, interaction::Interaction, user::{User, UserRef}}, services::discord::DISCORD_SERVICE};

#[allow(unused)]
pub struct CommandInteraction {
    pub data: CommandData, 
    pub id: Id<InteractionMarker>,
    pub locale: String,
    pub token: String,
    
    pub channel: Option<Channel>,
    pub channel_id: Option<Id<ChannelMarker>>,
    
    pub guild: Option<InteractionPartialGuild>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,

    pub message: Option<Message>,
    pub member: Option<PartialMember>,
    pub user: Option<User>,
    
    pub context: Option<InteractionContextType>,

    pub entitlements: Vec<Entitlement>,
    pub app_permissions: Option<Permissions>,
    pub application_id: Id<ApplicationMarker>,
    pub authorizing_integration_owners: ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>
}

impl CommandInteraction {
    pub (crate) fn with_inner_data(self) -> Option<Self> {
        let mut resolved = self.data.0.resolved;
        let guild_id = self.data.0.guild_id;
        let id = self.data.0.id;
        let kind = self.data.0.kind;
        let target_id = self.data.0.target_id;
        
        let new_data = self.data.0.options.into_iter().find_map(|opt| {
            match opt.value {
                TwilightCommandOptionValue::SubCommand(data) | 
                TwilightCommandOptionValue::SubCommandGroup(data) => {
                    Some(CommandData(TwilightCommandData {
                        name: opt.name,
                        options: data, // Questo è già un Vec, lo spostiamo direttamente
                        resolved: resolved.take(),
                        guild_id,
                        id,
                        kind,
                        target_id
                    }))
                },
                _ => None
            }
        });

        match new_data {
            Some(inner) => Some(Self { data: inner, ..self }),
            None => None,
        }
    }

    pub fn author<'a>(&'a self) -> Option<UserRef<'a>> {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().map(|a| a.into()),
            _ => self.user.as_ref().map(|a| a.into()),
        }
    }

    pub fn author_id(&self) -> Option<Id<UserMarker>> {
        self.author().map(|a| a.id)
    }

    /// Returns true if the interaction occurred within a guild (server) context.
    /// 
    /// This uses the `context` field if available, falling back to 
    /// checking if `guild_id` is present for older payloads.
    pub fn is_guild(&self) -> bool {
        match self.context {
            Some(InteractionContextType::Guild) => true,
            Some(_) => false,
            None => self.guild_id.is_some(),
        }
    }

    /// Returns true if the interaction occurred in a private context, 
    /// covering both direct messages and private group channels.
    pub fn is_private(&self) -> bool {
        self.is_private_channel() || self.is_bot_dm()
    }

    /// Returns true if the interaction occurred in a private group channel.
    pub fn is_private_channel(&self) -> bool {
        matches!(self.context, Some(InteractionContextType::PrivateChannel))
    }

    /// Returns true if the interaction occurred in a direct message (DM) 
    /// between the user and the bot.
    pub fn is_bot_dm(&self) -> bool {
        matches!(self.context, Some(InteractionContextType::BotDm))
    }

    pub async fn defer(&self, ephemeral: bool) -> BotResult<()> {
        let service = DISCORD_SERVICE.get().expect("DiscordService should be Some");

        let is_edit = if self.message.is_some() { true } else { false };

        service.defer(self.id, &self.token, is_edit, ephemeral).await?;

        Ok(())
    }

    pub async fn edit(&self, response: CommandResponse) -> BotResult<CommandResponse> {
        let service = DISCORD_SERVICE.get().expect("DiscordService should be Some");
        service.edit(self.application_id, &self.token, response).await?;

        Ok(CommandResponse::new())
    }
}

impl TryFrom<Interaction> for CommandInteraction {
    type Error = Error;

    fn try_from(mut value: Interaction) -> Result<Self, Self::Error> {
        let data = match value.data.take() {
            Some(InteractionData::ApplicationCommand(d)) => CommandData::from(*d),
            _ => return Err(Error::Generic("Expected ApplicationCommand".into())),
        };

        Ok(Self {
            application_id: value.application_id,
            authorizing_integration_owners: value.authorizing_integration_owners.clone(),
            channel: value.channel.take(),
            context: value.context.take(),
            entitlements: std::mem::take(&mut value.entitlements),
            guild: value.guild.take(),
            guild_locale: value.guild_locale.take(),
            locale: value.locale.take().expect("Locale should be always available"),
            data: data,
            id: value.id,
            token: std::mem::take(&mut value.token),
            #[allow(deprecated)]
            channel_id: value.channel_id,
            guild_id: value.guild_id,
            member: value.member.take(),
            message: value.message.take(),
            user: value.user.take().map(|u| u.into()),
            app_permissions: value.app_permissions
        })
    }
}