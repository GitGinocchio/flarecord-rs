use crate::{
    error::{BotResult, Error}, models::command::{
        Command, 
        CommandType, 
        Subcommand, 
        SubcommandGroup, 
        SubcommandGroupType, 
        context::CommandContext, 
        interaction::CommandInteraction, 
        response::CommandResponse
    }
};

pub (crate) struct CommandDispatcher;

impl CommandDispatcher {
    pub (crate) async fn dispatch(
        cmd: &CommandType,
        interaction: CommandInteraction,
        ctx: CommandContext
    ) -> BotResult<CommandResponse> {
        if let Some(group_name) = interaction.data.get_subcommand_group_name() {
            if let Some(group) = cmd.groups().iter().find(|g| g.name() == group_name) {
                let Some(inner_interaction) = interaction.with_inner_data() else {
                    return Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!")));
                };

                return Self::dispatch_group(group, inner_interaction, ctx).await
            }
        }

        if let Some(sub_name) = interaction.data.get_subcommand_name() {
            if let Some(sub) = cmd.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_interaction) = interaction.with_inner_data() else {
                    return Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!")));
                };

                return sub.execute(inner_interaction, ctx).await
            }
        }

        cmd.execute(interaction, ctx).await
    }

    async fn dispatch_group(
        group: &SubcommandGroupType,
        interaction: CommandInteraction,
        ctx: CommandContext
    ) -> BotResult<CommandResponse> {
        if let Some(sub_name) = interaction.data.get_subcommand_name() {
            if let Some(sub) = group.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_interaction) = interaction.with_inner_data() else {
                    return Err(Error::InvalidInteraction(format!("Missing inner data for the subcommand!")));
                };

                return sub.execute(inner_interaction, ctx).await
            }
        }

        Err(Error::CommandNotFound("Subcommand not found in group".into()))
    }
}