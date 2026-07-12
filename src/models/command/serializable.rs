use serde::{Serialize, Serializer};
use twilight_model::{
    application::command::{
        Command as TwilightCommand, 
        CommandOption as TwilightCommandOption, 
        CommandOptionType, 
        CommandType as TwilightCommandType
    }, 
    id::Id
};
use serde::ser::Error as SerdeError;

use crate::models::command::{Command, CommandType, Subcommand, SubcommandGroup, SubcommandType};

fn serialize_subcommands<S>(
    subcommands: &Vec<SubcommandType>
) -> Result<Vec<TwilightCommandOption>, S::Error> where S: Serializer {
    let mut serialized: Vec<TwilightCommandOption> = Vec::new();

    for subcommand in subcommands.iter() {
        let options: Vec<TwilightCommandOption> = subcommand.options()
            .map_err(|e| SerdeError::custom(format!("Error parsing options: {e}")))?
            .unwrap_or_default()
            .iter()
            .map(|opt| TwilightCommandOption::from(opt))
            .collect();

        let autocomplete = options
            .iter()
            .any(|o| o.autocomplete.map(|_v| true).unwrap_or(false));

        serialized.push(TwilightCommandOption {
            kind: CommandOptionType::SubCommand,
            name: subcommand.name(),
            description: subcommand.description(),
            options: if options.len() > 0 { Some(options) } else { None },
            autocomplete: if autocomplete { Some(true) } else { None },
            channel_types: None,
            choices: None,
            name_localizations: subcommand.name_localizations(),
            description_localizations: subcommand.description_localizations(),
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
            required: None
        });
    }
    
    Ok(serialized)
}

pub (crate) struct SerializableCommand<'a>(pub &'a CommandType);

impl<'a> Serialize for SerializableCommand<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut all_options: Vec<TwilightCommandOption> = self.0.options()
            .map_err(|e| SerdeError::custom(format!("Error parsing options: {e}")))?
            .unwrap_or_default()
            .iter()
            .map(|opt| TwilightCommandOption::from(opt))
            .collect();

        let groups = self.0.groups();
        for group in groups.iter() {
            let subcommands = group.subcommands();
            all_options.push(TwilightCommandOption {
                kind: CommandOptionType::SubCommandGroup,
                name: group.name(),
                description: group.description(),
                options: Some(serialize_subcommands::<S>(&subcommands)?),
                autocomplete: None,
                channel_types: None,
                choices: None,
                name_localizations: group.name_localizations(),
                description_localizations: group.description_localizations(),
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                required: None
            })
        }

        let subcommands = self.0.subcommands();
        all_options.append(&mut serialize_subcommands::<S>(&subcommands)?);

        let itypes = self.0.integration_types();
        let icontexts = self.0.interaction_contexts();

        let discord_cmd = TwilightCommand {
            name: self.0.name(),
            description: self.0.description(),
            options: all_options,
            kind: TwilightCommandType::ChatInput,
            application_id: None,
            default_member_permissions: self.0.default_member_permissions(),
            guild_id: self.0.guild_id(),
            id: None,
            nsfw: self.0.nsfw(),
            version: Id::new(1),
            name_localizations: self.0.name_localizations(),
            description_localizations: self.0.description_localizations(),
            contexts: if icontexts.is_empty() {
                None
            } else {
                Some(icontexts)
            },
            #[allow(deprecated)]
            dm_permission: None,
            integration_types: if itypes.is_empty() {
                None
            } else {
                Some(itypes)
            },
        };

        discord_cmd.serialize(serializer)
    }
}