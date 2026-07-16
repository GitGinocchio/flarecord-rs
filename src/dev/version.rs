use twilight_model::channel::message::embed::EmbedField;
use worker::WorkerVersionMetadata;

use crate::{error::BotResult, models::{color::Color, command::{Subcommand, context::CommandContext, interaction::CommandInteraction, response::CommandResponse}, embed::Embed}, services::discord::DiscordService};

pub struct VersionCommand;

impl Subcommand for VersionCommand {
    fn name(&self) -> String {
        "version".into()
    }

    fn description(&self) -> String {
        "A command used to get version information of the bot".into()
    }

    async fn execute(&self, _: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
        let mut embed = Embed::new();
        embed.set_title(Some(format!("Bot version information")));
        
        let Some(metadata_binding) = ctx.env.var("WORKER_METADATA_BINDING").ok() else {
            worker::console_warn!("WORKER_METADATA_BINDING env variable not set!");

            embed.set_color(Some(Color::from_rgb(255, 0, 0)));
            embed.set_description(Some(format!("An error occurred while gathering bot version information!")));

            return Ok(CommandResponse::builder()
                .embed(embed)
                .ephemeral()
                .build())
        };

        let metadata: WorkerVersionMetadata = ctx.env.get_binding::<WorkerVersionMetadata>(
            &metadata_binding.to_string()
        )?;

        let timestamp = metadata.timestamp();
        let id = metadata.id();
        let tag = metadata.tag();

        embed.set_color(Some(Color::from_rgb(0, 0, 255)));
        embed.add_field(EmbedField { inline: false, name: "Id".into(), value: id });
        embed.add_field(EmbedField { inline: false, name: "Tag".into(), value: tag });
        embed.add_field(EmbedField { inline: false, name: "Timestamp".into(), value: format!("<t:{timestamp}:f>") });

        Ok(CommandResponse::builder()
            .embed(embed)
            .ephemeral()
            .build())
    }
}