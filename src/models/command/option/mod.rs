use twilight_model::{
    application::command::{
        CommandOption as TwilightCommandOption, 
        CommandOptionType
    }, 
    channel::ChannelType
};

use crate::{error::{Error, BotResult}, models::command::option::builder::CommandOptionBuilder};

pub mod builder;
pub mod value;

pub struct CommandOption {
    name: String,
    description: String,
    autocomplete: Option<bool>,
    channel_types: Option<Vec<ChannelType>>,
    required: Option<bool>,
    kind: CommandOptionType
}

impl CommandOption {
    fn new(name: impl Into<String>, description: impl Into<String>, kind: CommandOptionType) -> Self {
        Self {
            name: name.into(), 
            description: description.into(),
            kind: kind,
            autocomplete: None,
            channel_types: None,
            required: None
        }
    }

    pub fn builder(name: impl Into<String>, description: impl Into<String>, kind: CommandOptionType) -> CommandOptionBuilder {
        CommandOptionBuilder::new(name, description, kind)
    }

    pub fn string(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::string(name, description).build()
    }

    pub fn bool(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::bool(name, description).build()
    }

    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::integer(name, description).build()
    }

    pub fn number(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::number(name, description).build()
    }

    pub fn mentionable(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::mentionable(name, description).build()
    }

    pub fn user(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::user(name, description).build()
    }

    pub fn role(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::role(name, description).build()
    }

    pub fn channel(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        let mut command_option = CommandOptionBuilder::channel(name, description).build()?;
        command_option.channel_types = Some(vec![]);
        Ok(command_option)
    }

    pub fn attachment(name: impl Into<String>, description: impl Into<String>) -> BotResult<Self> {
        CommandOptionBuilder::attachment(name, description).build()
    }

    pub fn set_required(&mut self, required: bool) {
        self.required = Some(required);
    }

    pub fn set_autocomplete(&mut self, autocomplete: bool) {
        self.autocomplete = Some(autocomplete);
    }
    
    pub fn set_channel_types(&mut self, channel_types: Vec<ChannelType>) -> BotResult<()> {
        if self.kind != CommandOptionType::Channel {
            return Err(Error::InvalidOptionType("Channel types are only valid for Channel options".into()));
        }

        self.channel_types = Some(channel_types);
        Ok(())
    }

    pub fn validate(&self) -> BotResult<()> {
        if self.name.is_empty() { 
            return Err(Error::InvalidOptionName(self.name.clone())) 
        }
        if self.name.chars().count() > 32 { 
            return Err(Error::InvalidOptionName(self.name.clone())) 
        }
        
        if self.description.chars().count() > 100 { 
            return Err(Error::InvalidOptionDescription(self.description.clone())) 
        }

        if self.kind != CommandOptionType::Channel && self.channel_types.is_some() {
            return Err(Error::InvalidOptionType("Channel types only for Channel".into()));
        }

        Ok(())
    }
}

// Dovrebbe diventare un try_from e fare un value.validate()?
impl From<CommandOption> for TwilightCommandOption {
    fn from(value: CommandOption) -> Self {
        TwilightCommandOption {
            name: value.name,
            description: value.description,
            autocomplete: value.autocomplete,
            channel_types: value.channel_types,
            required: value.required,
            kind: value.kind,
            choices: None,
            name_localizations: None,
            description_localizations: None,
            options: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
        }
    }
}

impl From<&CommandOption> for TwilightCommandOption {
    fn from(value: &CommandOption) -> Self {
        TwilightCommandOption {
            name: value.name.clone(),
            description: value.description.clone(),
            autocomplete: value.autocomplete,
            channel_types: value.channel_types.clone(),
            required: value.required,
            kind: value.kind,
            choices: None,
            name_localizations: None,
            description_localizations: None,
            options: None,
            max_length: None,
            max_value: None,
            min_length: None,
            min_value: None,
        }
    }
}