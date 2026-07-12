use crate::{error::{BotResult, Error}, models::command::{Subcommand, context::CommandContext, interaction::CommandInteraction, response::CommandResponse}, services::discord::DiscordService};

pub struct SyncCommand;

impl Subcommand for SyncCommand {
    fn name(&self) -> String {
        "sync".into()
    }

    fn description(&self) -> String {
        "A command used to sync newly created commands with the discord api".into()
    }

    async fn execute(&self, interaction: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
        let token = ctx.env.secret("DISCORD_BOT_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        let service = DiscordService::new(token);

        service.update_global_commands(interaction.application_id).await?;

        Ok(CommandResponse::builder()
            .ephemeral()
            .content(format!("Commands sync completed successfully!"))
            .build())
    }
}