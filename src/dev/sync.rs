use async_trait::async_trait;

use crate::{error::BotResult, models::command::{Subcommand, context::CommandContext, interaction::CommandInteraction, response::CommandResponse}};





pub struct SyncCommand;

#[async_trait(?Send)]
impl Subcommand for SyncCommand {
    fn name(&self) -> String {
        "sync".into()
    }

    fn description(&self) -> String {
        "A command used to sync newly created commands with the discord api".into()
    }

    async fn execute(&self, interaction: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
        ctx.discord.update_global_commands(interaction.application_id).await?;
        Ok(CommandResponse::builder()
            .content(format!("Commands sync completed successfully!"))
            .build())
    }
}