use twilight_model::application::command::CommandOptionType;
use twilight_model::channel::ChannelType;

use crate::models::command::option::CommandOption;
use crate::error::BotResult;

pub struct CommandOptionBuilder(CommandOption);

#[allow(unused)]
impl CommandOptionBuilder {
    pub fn new(name: impl Into<String>, description: impl Into<String>, kind: CommandOptionType) -> Self {
        Self(CommandOption::new(name, description, kind))
    }

    pub fn string(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::String))
    }

    pub fn bool(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::Boolean))
    }

    pub fn integer(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::Integer))
    }

    pub fn number(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::Number))
    }

    pub fn mentionable(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::Mentionable))
    }

    pub fn user(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::User))
    }

    pub fn role(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::Role))
    }

    pub fn channel(name: impl Into<String>, description: impl Into<String>) -> Self {
        let mut command_option = Self(CommandOption::new(name, description, CommandOptionType::Channel));
        command_option.0.channel_types = Some(vec![]);
        command_option
    }

    pub fn attachment(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self(CommandOption::new(name, description, CommandOptionType::Attachment))
    }

    pub fn autocomplete(mut self) -> Self {
        self.0.set_autocomplete(true);
        self
    }

    pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
        self.0.set_channel_types(channel_types);
        self
    }

    pub fn required(mut self) -> Self {
        self.0.set_required(true);
        self
    }

    pub fn build(self) -> BotResult<CommandOption> {
        self.0.validate()?;
        Ok(self.0)
    }
}