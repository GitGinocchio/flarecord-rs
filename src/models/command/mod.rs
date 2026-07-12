use std::{collections::HashMap, sync::Arc};

use async_trait::async_trait;
use twilight_model::{application::interaction::InteractionContextType, guild::Permissions, id::{Id, marker::GuildMarker}, oauth::ApplicationIntegrationType};

use crate::{
    error::{BotResult, Error}, 
    models::{
        autocomplete::{
            context::AutocompleteContext, 
            interaction::AutocompleteInteraction, response::AutocompleteResponse
        }, 
        command::{
            context::CommandContext, interaction::CommandInteraction, option::CommandOption, response::CommandResponse
        }
    }
};

pub (crate) mod dispatcher;
pub (crate) mod serializable;

pub mod data;
pub mod response;
pub mod option;
pub mod interaction;
pub mod context;

pub type CommandType = Arc<dyn Command>;

pub type CommandOptions = Option<Vec<CommandOption>>;

pub type SubcommandType = Box<dyn Subcommand>;
pub type SubcommandGroupType = Box<dyn SubcommandGroup>;

#[async_trait(?Send)]
#[allow(unused)]
pub trait Command: Send + Sync {
    fn name(&self) -> String;
    fn name_localizations(&self) -> Option<HashMap<String, String>> { None }
    
    fn description(&self) -> String;
    fn description_localizations(&self) -> Option<HashMap<String, String>> { None }
    
    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn guild_id(&self) -> Option<Id<GuildMarker>> { None }
    fn nsfw(&self) -> Option<bool> { None }
    
    fn interaction_contexts(&self) -> Vec<InteractionContextType> { vec![] }
    fn integration_types(&self) -> Vec<ApplicationIntegrationType> { vec![] }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![] }
    fn groups(&self) -> Vec<SubcommandGroupType> { vec![] }

    fn options(&self) -> BotResult<CommandOptions> { Ok(None) }

    async fn autocomplete(&self, interaction: AutocompleteInteraction, ctx: AutocompleteContext) -> BotResult<AutocompleteResponse> {
        Err(Error::AutocompleteNotImplemented(self.name()))
    }

    async fn execute(&self, interaction: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
        Err(Error::ExecuteNotImplemented(self.name()))
    }
}

#[async_trait(?Send)]
#[allow(unused)]
pub trait Subcommand: Send + Sync {
    fn name(&self) -> String;
    fn name_localizations(&self) -> Option<HashMap<String, String>> { None }
    
    fn description(&self) -> String;
    fn description_localizations(&self) -> Option<HashMap<String, String>> { None }

    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn options(&self) -> BotResult<CommandOptions> { Ok(None) }

    async fn autocomplete(&self, interaction: AutocompleteInteraction, ctx: AutocompleteContext) -> BotResult<AutocompleteResponse> {
        Err(Error::AutocompleteNotImplemented(self.name()))
    }

    async fn execute(&self, interaction: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse>;
}

#[async_trait(?Send)]
#[allow(unused)]
pub trait SubcommandGroup: Send + Sync {
    fn name(&self) -> String;
    fn name_localizations(&self) -> Option<HashMap<String, String>> { None }
    
    fn description(&self) -> String;
    fn description_localizations(&self) -> Option<HashMap<String, String>> { None }

    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn subcommands(&self) -> Vec<SubcommandType> { vec![] }
}

pub struct CommandHandler<F, Fut> {
    pub name: String,
    pub description: String,
    pub handler: F,
    _marker: std::marker::PhantomData<Fut>,
}

impl<F, Fut> CommandHandler<F, Fut> {
    pub fn new(name: String, description: String, handler: F) -> Self {
        Self {
            name,
            description,
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<F, Fut> Command for CommandHandler<F, Fut> 
where 
    F: Fn(CommandInteraction, CommandContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<CommandResponse>> + Send + Sync + 'static,
{
    fn name(&self) -> String { self.name.clone() }
    fn description(&self) -> String { self.description.clone() }

    async fn execute(&self, interaction: CommandInteraction, ctx: CommandContext) -> BotResult<CommandResponse> {
        (self.handler)(interaction, ctx).await
    }
}