use std::any::Any;

use twilight_model::{
    channel::message::{
        Embed as TwilightEmbed,
        Component as TwilightComponent, 
        MessageFlags
    }, 
    http::{
        interaction::{
            InteractionResponse as TwilightCommandResponse,
            InteractionResponseType
        }
    }
};

use crate::{
    models::{
        attachment::outgoing::Attachment, command::response::builder::CommandResponseBuilder, components::{Component, id::{get_component_id, get_component_id_from_type_id}, layout::RootComponent}, embed::Embed
    }, traits::component::{
        IntoComponent, 
        IntoTwilight
    }
};

pub mod builder;

#[derive(serde::Serialize, Default, Debug)]
pub (crate) struct CommandResponseUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<TwilightEmbed>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<TwilightComponent>>,
}

#[derive(Debug)]
pub struct CommandResponse(pub (crate) TwilightCommandResponse);

impl CommandResponse {
    pub fn new() -> Self {
        Self::empty()
    }

    pub fn new_with_kind(kind: InteractionResponseType) -> Self {
        let mut empty = Self::empty();
        empty.0.kind = kind;
        empty
    }

    pub fn builder() -> CommandResponseBuilder {
        CommandResponseBuilder::new()
    }

    pub fn empty() -> Self {
        Self(TwilightCommandResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: None
        })
    }

    /// Note: content will be ignored when using components V2
    pub fn set_content(&mut self, content: impl Into<String>) {
        self.0.data.get_or_insert_default().content = Some(content.into());
    }

    pub fn add_embed(&mut self, embed: Embed) {
        self.0.data.get_or_insert_default()
            .embeds
            .get_or_insert_default()
            .push(embed.into_twilight())
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.0.data.get_or_insert_default()
            .attachments
            .get_or_insert_default()
            .push(attachment.into_twilight())
    }

    /// Adds a component to the response.
    ///
    /// The `component` argument can be any type that implements [`IntoComponent`].
    /// 
    /// This includes:
    /// - **Layout Components**: Any type that can be converted into a [`LayoutComponent`], 
    ///   specifically: [`ActionRow`], [`Container`], [`Section`], or [`Separator`].
    /// - **Custom Components**: Any type implementing the [`Component`] trait.
    pub fn add_component(&mut self, component: impl IntoComponent) {
        let component = component.into_component();

        let component_id = get_component_id_from_type_id(component.type_id());
        let mut root = RootComponent::new();
        component.build(&mut root);

        root.set_component_id(component_id);
        root.assign_ids();

        if root.require_components_v2() {
            self.0.data.get_or_insert_default()
                .flags
                .get_or_insert(MessageFlags::empty())
                .insert(MessageFlags::IS_COMPONENTS_V2);
        }

        let mut components: Vec<TwilightComponent> = root.into_twilight();
        self.0.data.get_or_insert_default()
            .components
            .get_or_insert_default()
            .append(&mut components)
    }

    pub fn set_ephemeral(&mut self, ephemeral: bool) {
        self.0.data.get_or_insert_default()
            .flags
            .get_or_insert(MessageFlags::empty())
            .set(MessageFlags::EPHEMERAL, ephemeral);
    }

    pub (crate) fn as_update(self) -> CommandResponseUpdate {
        let Some(mut data) = self.0.data else {
            return CommandResponseUpdate::default();
        };

        CommandResponseUpdate {
            flags: data.flags.take(),
            content: data.content.take(),
            embeds: data.embeds.take(),
            components: data.components.take()
        }
    }
}

impl IntoTwilight<TwilightCommandResponse> for CommandResponse {
    fn into_twilight(self) -> TwilightCommandResponse {
        self.0
    }
}