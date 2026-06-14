use crate::{models::{attachment::outgoing::Attachment, embed::Embed}, traits::component::IntoComponent};

use super::CommandResponse;

pub struct CommandResponseBuilder(CommandResponse);

impl CommandResponseBuilder {
    pub fn new() -> Self {
        Self(CommandResponse::new())
    }

    pub fn build(self) -> CommandResponse {
        self.0
    }

    /// Note: content will be ignored when using components V2
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.set_content(content);
        self
    }

    pub fn ephemeral(mut self) -> Self {
        self.0.set_ephemeral(true);
        self
    }

    pub fn attachments(mut self, attachments: Vec<Attachment>) -> Self {
        for attachment in attachments {
            self.0.add_attachment(attachment);
        }
        self
    }

    pub fn attachment(mut self, attachment: Attachment) -> Self {
        self.0.add_attachment(attachment);
        self
    }

    /// Adds a component to the response.
    ///
    /// The `component` argument can be any type that implements [`IntoComponent`].
    /// 
    /// This includes:
    /// - **Layout Components**: Any type that can be converted into a [`LayoutComponent`], 
    ///   specifically: [`ActionRow`], [`Container`], [`Section`], or [`Separator`].
    /// - **Custom Components**: Any type implementing the [`Component`] trait.
    pub fn component(mut self, component: impl IntoComponent) -> Self {
        self.0.add_component(component);
        self
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        for embed in embeds {
            self.0.add_embed(embed);
        }
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.0.add_embed(embed);
        self
    }
}