use flarecord::{models::command::{IntoSubcommandGroup, SubcommandGroupType, SubcommandType}, prelude::*};

use crate::commands::mycommand::subcommand::MySubcommand;
use crate::commands::mycommand::mysubgroup::MySubcommandGroup;

pub mod mysubgroup;
pub mod subcommand;


#[flarecord::command]
impl Command for MyCommand {
    fn name(&self) -> String {
        "mycommand".into()
    }

    fn description(&self) -> String {
        "My command that contains a subcommand".into()
    }

    fn groups(&self) -> Vec<SubcommandGroupType> { vec![
        MySubcommandGroup.into_subcommand_group()
    ]}

    fn subcommands(&self) -> Vec<SubcommandType> { vec![
        MySubcommand.into_subcommand(),
    ]}

    /* Execute method will not receive interactions anymore when using subcommands!
    async fn execute(&self, interaction: CommandInteraction, _ctx: CommandContext) -> BotResult<CommandResponse> {
        // ...
    }
    */
}