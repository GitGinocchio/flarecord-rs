use std::{collections::HashMap, sync::Arc};

use crate::{
    CommandRegistration, bot::Bot, dev::DevCommands, error::BotResult, models::{
        command::{
            Command, CommandHandler, CommandType, IntoCommand, context::CommandContext, interaction::CommandInteraction, response::CommandResponse
        }, components::{Component, ComponentType}, modals::{IntoModal, Modal, ModalType}
    }, traits::component::IntoComponent
};

pub struct BotBuilder {
    pub (crate) commands: HashMap<String, CommandType>,
    pub (crate) components: HashMap<String, ComponentType>,
    pub (crate) modals: HashMap<String, ModalType>
}

impl BotBuilder {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            components: HashMap::new(),
            modals: HashMap::new()
        }
    }

    pub fn enable_dev_commands(self) -> Self {
        self.register_command(DevCommands)
    }

    pub fn register_component(mut self, component: impl Component + 'static) -> Self {
        self.components.insert(component.id(), component.into_component());
        self
    }

    pub fn register_modal(mut self, modal: impl Modal + 'static) -> Self {
        self.modals.insert(modal.id(), modal.into_modal());
        self
    }

    pub fn register_command(mut self, command: impl Command + 'static) -> Self {
        self.commands.insert(command.name(), command.into_command());
        self
    }

    pub fn register_command_handler<F, Fut>(mut self, 
        name: impl Into<String>, 
        description: impl Into<String>, 
        handler: F
    ) -> Self
    where 
        F: Fn(CommandInteraction, CommandContext) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = BotResult<CommandResponse>> + Send + Sync + 'static,
    {
        let handler = CommandHandler::new(name.into(), description.into(), handler);
        self.commands.insert(handler.name.clone(), handler.into_command());
        self
    }

    pub fn build(mut self) -> Arc<Bot> {
        for reg in inventory::iter::<CommandRegistration> {
            let cmd = (reg.constructor)();
            self.commands.insert(cmd.name().to_string(), cmd);
        }

        Bot::from(self).set_global();

        Bot::get_global()
    }
}