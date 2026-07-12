use flarecord::models::command::{IntoSubcommand, SubcommandGroup, SubcommandType};

pub mod subcommand;

use crate::commands::mycommand::mysubgroup::subcommand::MySubcommand;


pub struct MySubcommandGroup;

impl SubcommandGroup for MySubcommandGroup {
    fn name(&self) -> String {
        "mysubgroup".into()
    }

    fn description(&self) -> String {
        "My subgroup that contains a subcommand".into()
    }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![
        MySubcommand.into_subcommand()
    ]}
}