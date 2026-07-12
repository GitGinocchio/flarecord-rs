use std::ops::{Deref, DerefMut};

use twilight_model::{application::interaction::{Interaction as TwilightInteraction, InteractionType}, http::interaction::{InteractionResponse, InteractionResponseType}};
use worker::{Env, Response};

use crate::{bot::{Bot, state::BotState}, error::{BotResult, Error}, models::{autocomplete::{context::AutocompleteContext, dispatcher::AutocompleteDispatcher, interaction::AutocompleteInteraction}, command::{context::CommandContext, dispatcher::CommandDispatcher, interaction::CommandInteraction}, components::{context::ComponentContext, dispatcher::ComponentDispatcher, interaction::ComponentInteraction}, modals::{Modal, context::ModalContext, interaction::ModalInteraction}}, services::discord::DiscordService, traits::component::IntoTwilight};

#[allow(unused)]
pub (crate) struct Interaction(TwilightInteraction);

#[allow(unused)]
impl Interaction {
    pub (crate) async fn perform(self, env: Env) -> BotResult<Response> {
        match self.kind {
            InteractionType::ApplicationCommandAutocomplete => self.handle_autocomplete(env).await,
            InteractionType::ApplicationCommand => self.handle_command(env).await,
            InteractionType::MessageComponent => self.handle_component(env).await,
            InteractionType::ModalSubmit => self.handle_modal_submit(env).await,
            InteractionType::Ping => self.handle_ping().await,
            _ => Ok(Response::empty()?)
        }
    }

    async fn handle_ping(&self) -> BotResult<Response> {
        let response = InteractionResponse {
            kind: InteractionResponseType::Pong,
            data: None
        };
        
        let value = serde_json::to_value(response)?;
        Response::from_json(&value).map_err(Error::WorkerError)
    }

    async fn handle_command(self, env: Env) -> BotResult<Response> {
        let command_interaction = CommandInteraction::try_from(self)?;
        
        let bot = Bot::get_global();

        let Some(command) = bot.commands.get(&command_interaction.data.0.name) else {
            return Err(Error::CommandNotFound(format!("{}", command_interaction.data.0.name)))
        };

        let discord_service = DiscordService::get_or_init(command_interaction.token.clone());

        let bot_state = BotState::new(bot.clone());
        let ctx = CommandContext::new(bot_state, env, discord_service);

        match CommandDispatcher::dispatch(command, command_interaction, ctx).await {
            Err(e) => Ok(e.as_response()?),
            Ok(response) => {
                let value = serde_json::to_value::<InteractionResponse>(response.into_twilight())
                    .map_err(Error::JsonFailed)?;

                worker::console_debug!("command_response: {value:?}");

                Response::from_json(&value).map_err(Error::WorkerError)
            }
        }
    }

    async fn handle_autocomplete(self, env: Env) -> BotResult<Response> {
        let autocomplete_interaction = AutocompleteInteraction::try_from(self)?;

        let bot = Bot::get_global();
        let Some(command) = bot.commands.get(&autocomplete_interaction.data.0.name) else {
            return Err(Error::CommandNotFound(format!("{}", autocomplete_interaction.data.0.name)))
        };

        let discord_service = DiscordService::get_or_init(autocomplete_interaction.token.clone());

        let bot_state = BotState::new(bot.clone());
        let ctx = AutocompleteContext::new(bot_state, env, discord_service);

        match AutocompleteDispatcher::dispatch(command, autocomplete_interaction, ctx).await {
            Err(e) => Ok(e.as_response()?),
            Ok(response) => {
                let value = serde_json::to_value::<InteractionResponse>(response.into())
                    .map_err(Error::JsonFailed)?;

                Response::from_json(&value).map_err(Error::WorkerError)
            },
        }
    }

    async fn handle_modal_submit(self, env: Env) -> BotResult<Response> {
        let modal_interaction = ModalInteraction::try_from(self)?;

        let bot = Bot::get_global();
        let Some(modal) = bot.modals.get(&modal_interaction.data.custom_id) else {
            return Err(Error::ModalNotFound(format!("{}", modal_interaction.data.custom_id)))
        };

        let discord_service = DiscordService::get_or_init(modal_interaction.token.clone());

        let bot_state = BotState::new(bot.clone());
        let ctx = ModalContext::new(bot_state, env, discord_service);

        match modal.on_submit(modal_interaction, ctx).await {
            Ok(response) => Ok(Response::empty()?),
            Err(e) => Ok(e.as_response()?)
        }
    }

    async fn handle_component(self, env: Env) -> BotResult<Response> {
        let component_interaction = ComponentInteraction::try_from(self)?;

        let bot = Bot::get_global();
        let Some(component) = bot.components.get(&component_interaction.data.custom_id) else {
            return Err(Error::ComponentNotFound(format!("{}", component_interaction.data.custom_id)))
        };

        let discord_service = DiscordService::get_or_init(component_interaction.token.clone());
        let ctx = ComponentContext::new(bot.clone(), env, discord_service);

        match ComponentDispatcher::dispatch(component, component_interaction, ctx).await {
            Err(e) => Ok(e.as_response()?),
            Ok(response) => {
                let value = serde_json::to_value::<InteractionResponse>(response.into_twilight())
                    .map_err(Error::JsonFailed)?;

                Response::from_json(&value).map_err(Error::WorkerError)
            }
        }
    }
}

impl From<TwilightInteraction> for Interaction {
    fn from(value: TwilightInteraction) -> Self {
        Self(value)
    }
}

impl Deref for Interaction {
    type Target = TwilightInteraction;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Interaction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}