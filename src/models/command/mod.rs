use std::{collections::HashMap, sync::Arc};

use dynosaur::dynosaur;
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

pub type CommandType = Arc<DynCommand<'static>>;
pub type SubcommandType = Arc<DynSubcommand<'static>>;
pub type SubcommandGroupType = Arc<DynSubcommandGroup<'static>>;

pub type CommandOptions = Option<Vec<CommandOption>>;

#[dynosaur(DynCommand = dyn(box) Command)]
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

    fn autocomplete(
        &self, 
        _interaction: AutocompleteInteraction, 
        _ctx: AutocompleteContext
    ) -> impl std::future::Future<Output = BotResult<AutocompleteResponse>> + Send {async {
        Err(Error::AutocompleteNotImplemented(self.name()))
    } }

    fn execute(
        &self, 
        _interaction: CommandInteraction, 
        _ctx: CommandContext
    ) -> impl std::future::Future<Output = BotResult<CommandResponse>> + Send {async {
        Err(Error::ExecuteNotImplemented(self.name()))
    } }
}


#[dynosaur(DynSubcommand = dyn(box) Subcommand)]
pub trait Subcommand: Send + Sync {
    fn name(&self) -> String;
    fn name_localizations(&self) -> Option<HashMap<String, String>> { None }
    
    fn description(&self) -> String;
    fn description_localizations(&self) -> Option<HashMap<String, String>> { None }

    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn options(&self) -> BotResult<CommandOptions> { Ok(None) }

    fn autocomplete(
        &self, 
        _interaction: AutocompleteInteraction, 
        _ctx: AutocompleteContext
    ) -> impl std::future::Future<Output = BotResult<AutocompleteResponse>> + Send {async {
        Err(Error::AutocompleteNotImplemented(self.name()))
    } }

    fn execute(
        &self, 
        _interaction: CommandInteraction, 
        _ctx: CommandContext
    ) -> impl std::future::Future<Output = BotResult<CommandResponse>> + Send {async {
        Err(Error::ExecuteNotImplemented(self.name()))
    } }
}

#[dynosaur(DynSubcommandGroup = dyn(box) SubcommandGroup)]
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

pub trait IntoSubcommandGroup {
    fn into_subcommand_group(self) -> SubcommandGroupType;
}

impl<S: SubcommandGroup + 'static> IntoSubcommandGroup for S {
    fn into_subcommand_group(self) -> SubcommandGroupType {
        DynSubcommandGroup::new_arc(self)
    }
}

pub trait IntoSubcommand {
    fn into_subcommand(self) -> SubcommandType;
}

impl<S: Subcommand + 'static> IntoSubcommand for S {
    fn into_subcommand(self) -> SubcommandType {
        DynSubcommand::new_arc(self)
    }
}

pub trait IntoCommand {
    fn into_command(self) -> CommandType;
}

impl<S: Command + 'static> IntoCommand for S {
    fn into_command(self) -> CommandType {
        DynCommand::new_arc(self)
    }
}