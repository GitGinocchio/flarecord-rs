use worker::WorkerVersionMetadata;

use crate::{error::{BotResult}, models::{color::Color, command::{Subcommand, context::CommandContext, interaction::CommandInteraction, response::CommandResponse}, components::{content::{text_display::TextDisplay, thumbnail::Thumbnail}, layout::{container::Container, section::Section}}, embed::Embed}};

pub struct VersionCommand;

impl Subcommand for VersionCommand {
    fn name(&self) -> String {
        "version".into()
    }

    fn description(&self) -> String {
        "A command used to get version information of the bot".into()
    }

    async fn execute(&self, interaction: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
        interaction.defer(true).await?;
        
        let Some(metadata_binding) = ctx.env.var("WORKER_METADATA_BINDING").ok() else {
            worker::console_warn!("WORKER_METADATA_BINDING env variable not set!");

            let mut embed = Embed::new();
            embed.set_title(Some(format!("Bot version information")));
            embed.set_color(Some(Color::from_rgb(255, 0, 0)));
            embed.set_description(Some(format!("An error occurred while gathering bot version information!")));

            let response = CommandResponse::builder()
                .embed(embed)
                .ephemeral()
                .build();

            return interaction.edit(response).await;
        };

        let metadata: WorkerVersionMetadata = ctx.env.get_binding::<WorkerVersionMetadata>(
            &metadata_binding.to_string()
        )?;

        let datetime = metadata.timestamp();
        let id = metadata.id();
        let tag = metadata.tag();

        let date = worker::Date::new(worker::DateInit::String(datetime.clone()));
        let timestamp = (date.as_millis() / 1000) as u64;


        let text_display = TextDisplay::new()
            .heading(1, "Bot version information")
            .newline()
            .bold("Build Id:")
            .paragraph(&format!("`{id}`"))
            .newline()
            .bold("Build Tag:")
            .paragraph(if tag.is_empty() { "`<undefined>`".into() } else { format!("`{tag}`") })
            .newline()
            .bold("Build Timestamp:")
            .paragraph(format!("<t:{}:f>", timestamp))
            .newline()
            .bold("Time since build:")
            .paragraph(format!("<t:{}:R>", timestamp));

        let section = Section::new()
            .accessory(Thumbnail::new("https://img.icons8.com/external-tal-revivo-shadow-tal-revivo/96/external-cloudflare-provides-content-delivery-network-services-ddos-mitigation-logo-shadow-tal-revivo.png"))
            .component(text_display)
            .build();

        let color = Color::from_hex("#ED7E1F").unwrap_or(Color::from_rgb(0, 0, 255));

        let container = Container::new()
            .accent_color(color)
            .add(section);


        let response = CommandResponse::builder()
            .component(container)
            .ephemeral()
            .build();

        interaction.edit(response).await
    }
}