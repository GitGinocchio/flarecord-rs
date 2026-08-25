use std::{any::Any, collections::HashMap, sync::Arc};

use crate::{
    CommandRegistration, bot::{Bot, commands::DefaultBotCommands}, error::BotResult, models::{
        command::{
            Command, 
            CommandHandler, 
            CommandType, 
            IntoCommand, 
            context::CommandContext, 
            interaction::CommandInteraction, 
            response::CommandResponse
        }, components::{Component, ComponentType}, modals::{
            IntoModal, 
            Modal, 
            ModalType
        }
    }, traits::component::IntoComponent, utils::get_id_from_type_id
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

    pub fn enable_bot_commands(self) -> Self {
        self.register_command(DefaultBotCommands)
    }

    pub fn register_component<T: Component + 'static>(mut self, component: T) -> Self {
        let component = component.into_component();
        let component_id = get_id_from_type_id(component.type_id());
        self.components.insert(component_id, component);
        self
    }

    pub fn register_modal<T: Modal + 'static>(mut self, modal: T) -> Self {
        let modal = modal.into_modal();
        //let modal_id = get_id_from_type_id(modal.type_id());
        self.modals.insert(modal.name(), modal);
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
        // Register inventory commands
        for reg in inventory::iter::<CommandRegistration> {
            let cmd = (reg.constructor)();
            self.commands.insert(cmd.name().to_string(), cmd);
        }

        Bot::from(self).set_global();

        Bot::get_global()
    }
}