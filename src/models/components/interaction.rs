use twilight_model::{application::{interaction::{InteractionContextType, InteractionData, InteractionPartialGuild}, monetization::Entitlement}, channel::{Channel, Message, message::MessageFlags}, guild::{PartialMember, Permissions}, id::{AnonymizableId, Id, marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker}}, oauth::ApplicationIntegrationMap};

use crate::{
    error::{BotResult, Error}, models::{
        command::response::CommandResponse, 
        components::data::ComponentData, 
        interaction::Interaction, 
        user::{User, UserRef}
    }, 
    services::discord::DISCORD_SERVICE
};

#[allow(unused)]
pub struct ComponentInteraction {
    pub data: ComponentData, 
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

#[allow(unused)]
impl ComponentInteraction {
    pub fn author<'a>(&'a self) -> Option<UserRef<'a>> {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().map(|a| a.into()),
            _ => self.user.as_ref().map(|a| a.into()),
        }
    }

    pub fn author_id(&self) -> Option<Id<UserMarker>> {
        self.author().map(|a| a.id)
    }

    pub async fn defer(&self, ephemeral: bool) -> BotResult<()> {
        let service = DISCORD_SERVICE.get().expect("DiscordService should be Some");

        let is_edit = if self.message.is_some() { true } else { false };

        service.defer(self.id, &self.token, is_edit, ephemeral).await?;

        Ok(())
    }

    pub async fn edit(&self, mut response: CommandResponse) -> BotResult<CommandResponse> {
        let service = DISCORD_SERVICE.get().expect("DiscordService should be Some");

        if let Some(message) = &self.message && let Some(flags) = message.flags && flags.contains(MessageFlags::IS_COMPONENTS_V2) {
            response.0.data
                .get_or_insert_default()
                .flags
                .get_or_insert(MessageFlags::empty())
                .insert(MessageFlags::IS_COMPONENTS_V2);
        };

        service.edit(self.application_id, &self.token, response).await?;

        Ok(CommandResponse::new())
    }
}

impl TryFrom<Interaction> for ComponentInteraction {
    type Error = Error;

    fn try_from(mut value: Interaction) -> Result<Self, Self::Error> {
        let data = match value.data.take() {
            Some(InteractionData::MessageComponent(d)) => ComponentData::from(*d),
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
            message: value.message.take(),
            member: value.member.take(),
            user: value.user.take().map(|u| u.into()),
            app_permissions: value.app_permissions
        })
    }
}