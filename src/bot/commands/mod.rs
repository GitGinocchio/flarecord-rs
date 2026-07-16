use twilight_model::guild::Permissions;

use crate::{bot::commands::{sync::SyncCommand, version::VersionCommand}, models::command::{Command, IntoSubcommand, SubcommandType}};

pub mod sync;
pub mod version;

pub struct DefaultBotCommands;

impl Command for DefaultBotCommands {
    fn name(&self) -> String {
        "bot".into()
    }

    fn default_member_permissions(&self) -> Option<Permissions> {
        Some(Permissions::ADMINISTRATOR)
    }

    fn description(&self) -> String {
        "Useful set of commands that can be used to manage the bot".into()
    }

    fn subcommands(&self) -> Vec<SubcommandType> {
        vec![
            SyncCommand.into_subcommand(),
            VersionCommand.into_subcommand()
        ]
    }
}