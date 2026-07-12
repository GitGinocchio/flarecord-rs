use std::ops::Deref;

use twilight_model::id::{Id, marker::UserMarker};
use twilight_model::user::User as TwilightUser;

use crate::services::discord::DISCORD_SERVICE;
use crate::traits::resolvable::Resolvable;
use crate::error::{Error, BotResult};

#[allow(unused)]
pub struct User(TwilightUser);
pub struct UserRef<'a>(&'a TwilightUser);

#[allow(unused)]
pub trait UserTrait {
    fn mention(&self) -> String;
}

impl UserTrait for User {
    fn mention(&self) -> String {
        format!("<@{}>", self.0.id)
    }
}

impl<'a> UserTrait for UserRef<'a> {
    fn mention(&self) -> String {
        format!("<@{}>", self.0.id)
    }
}

impl From<TwilightUser> for User {
    fn from(value: TwilightUser) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a TwilightUser> for UserRef<'a> {
    fn from(value: &'a TwilightUser) -> Self {
        Self(&value)
    }
}

impl<'a> From<&'a User> for UserRef<'a> {
    fn from(value: &'a User) -> Self {
        Self(&value)
    }
}

impl Deref for User {
    type Target = TwilightUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Deref for UserRef<'a> {
    type Target = TwilightUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Resolvable<User> for Id<UserMarker> {
    async fn resolve(&self) -> BotResult<User> {
        let discord = DISCORD_SERVICE
            .get()
            .ok_or(Error::Generic("Discord Service not initiliazed!".into()))?;

        Ok(discord.fetch_user(self).await?)
    }
}