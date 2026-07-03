use async_trait::async_trait;
use twilight_model::guild::Permissions;

use crate::{dev::sync::SyncCommand, models::command::{Command, SubcommandType}};

pub mod sync;


pub struct DevCommands;

#[async_trait(?Send)]
impl Command for DevCommands {
    fn name(&self) -> String {
        "dev".into()
    }

    fn default_member_permissions(&self) -> Option<Permissions> {
        Some(Permissions::ADMINISTRATOR)
    }

    fn description(&self) -> String {
        "Useful set of commands that can be used to manage the bot while in development mode".into()
    }

    fn subcommands(&self) -> Vec<SubcommandType> {
        vec![
            Box::new(SyncCommand)
        ]
    }
}