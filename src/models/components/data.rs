use std::ops::Deref;

use twilight_model::{
    application::interaction::{InteractionChannel, InteractionDataResolved, InteractionMember, message_component::MessageComponentInteractionData as TwilightComponentData}, channel::{Message, message::component::ComponentType as TwilightComponentType}, guild::Role, id::{Id, marker::{AttachmentMarker, ChannelMarker, MessageMarker, RoleMarker, UserMarker}}
};

use crate::models::{attachment::incoming::IncomingAttachmentRef, user::UserRef};


pub struct ComponentData(TwilightComponentData);

impl ComponentData {
    pub fn custom_id(&self) -> &str {
        self.0.custom_id.as_str()
    }

    pub fn component_type(&self) -> TwilightComponentType {
        self.0.component_type
    }

    pub fn selected_values(&self) -> &[String] {
        self.0.values.as_slice()
    }

    pub fn resolved(&self) -> Option<&InteractionDataResolved> {
        self.0.resolved.as_ref()
    }

    /// Retrieves a resolved user by id.
    pub fn get_resolved_user<'a>(&'a self, id: Id<UserMarker>) -> Option<UserRef<'a>> {
        self.0.resolved.as_ref()?.users.get(&id).map(UserRef::from)
    }

    /// Retrieves a resolved member by id.
    pub fn get_resolved_member<'a>(&'a self, id: Id<UserMarker>) -> Option<&'a InteractionMember> {
        self.0.resolved.as_ref()?.members.get(&id)
    }

    /// Retrieves a resolved role by id.
    pub fn get_resolved_role<'a>(&'a self, id: Id<RoleMarker>) -> Option<&'a Role> {
        self.0.resolved.as_ref()?.roles.get(&id)
    }

    /// Retrieves a resolved message by id.
    pub fn get_resolved_message<'a>(&'a self, id: Id<MessageMarker>) -> Option<&'a Message> {
        self.0.resolved.as_ref()?.messages.get(&id.cast())
    }

    /// Retrieves a resolved channel by id.
    pub fn get_resolved_channel<'a>(&'a self, id: Id<ChannelMarker>) -> Option<&'a InteractionChannel> {
        self.0.resolved.as_ref()?.channels.get(&id)
    }

    /// Retrieves a resolved attachment by id.
    pub fn get_resolved_attachment<'a>(&'a self, id: Id<AttachmentMarker>) -> Option<IncomingAttachmentRef<'a>> {
        self.0.resolved.as_ref()?.attachments.get(&id).map(|a| a.into())
    }

}

impl From<TwilightComponentData> for ComponentData {
    fn from(value: TwilightComponentData) -> Self {
        Self(value)
    }
}

impl Deref for ComponentData {
    type Target = TwilightComponentData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}