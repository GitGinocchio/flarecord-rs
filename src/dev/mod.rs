use twilight_model::guild::Permissions;

use crate::{dev::{sync::SyncCommand, version::VersionCommand}, models::command::{Command, IntoSubcommand, SubcommandType}};

pub mod sync;
pub mod version;

pub struct DevCommands;

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
            SyncCommand.into_subcommand(),
            VersionCommand.into_subcommand()
        ]
    }
}