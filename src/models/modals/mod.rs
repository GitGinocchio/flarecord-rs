use std::sync::Arc;

use dynosaur::dynosaur;
use twilight_model::{application::interaction::InteractionContextType, guild::Permissions, oauth::ApplicationIntegrationType};

use crate::{
    error::{BotResult, Error}, 
    models::{
        modals::{
            context::ModalContext, 
            interaction::ModalInteraction
        }
    }
};

pub (crate) mod context;
pub (crate) mod data;
pub (crate) mod interaction;

pub type ModalType = Arc<DynModal<'static>>;
pub type SubmodalType = Arc<DynSubmodal<'static>>;

#[allow(async_fn_in_trait)]
#[dynosaur(DynModal = dyn(box) Modal)]
pub trait Modal: Send + Sync {
    fn name(&self) -> String;
    
    fn description(&self) -> String;
    
    fn default_member_permissions(&self) -> Option<Permissions> { None }

    fn interaction_contexts(&self) -> Vec<InteractionContextType> { vec![] }
    fn integration_types(&self) -> Vec<ApplicationIntegrationType> { vec![] }

    fn options(&self) -> BotResult<Option<Vec<crate::models::command::option::CommandOption>>> { Ok(None) }

    async fn on_submit(
        &self, 
        _interaction: ModalInteraction, 
        _ctx: ModalContext
    ) -> BotResult<()> {
        Err(Error::ExecuteNotImplemented(self.name()))
    }
}

pub struct ModalHandler<F, Fut> {
    pub name: String,
    pub description: String,
    pub handler: F,
    _marker: std::marker::PhantomData<Fut>,
}

impl<F, Fut> ModalHandler<F, Fut> {
    pub fn new(name: String, description: String, handler: F) -> Self {
        Self {
            name,
            description,
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<F, Fut> Modal for ModalHandler<F, Fut> 
where 
    F: Fn(ModalInteraction, ModalContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<()>> + Send + Sync + 'static,
{
    fn name(&self) -> String { self.name.clone() }
    fn description(&self) -> String { self.description.clone() }
    
    async fn on_submit(
        &self, 
        interaction: ModalInteraction, 
        ctx: ModalContext
    ) -> BotResult<()> {
        (self.handler)(interaction, ctx).await
    }
}

#[allow(async_fn_in_trait)]
#[dynosaur(DynSubmodal = dyn(box) Submodal)]
pub trait Submodal: Send + Sync {
    fn name(&self) -> String;
    
    fn description(&self) -> String;
    
    async fn on_submit(
        &self, 
        interaction: ModalInteraction, 
        ctx: ModalContext
    ) -> BotResult<()>;
}

pub trait IntoModal {
    fn into_modal(self) -> ModalType;
}

impl<M: Modal + 'static> IntoModal for M {
    fn into_modal(self) -> ModalType {
        DynModal::new_arc(self)
    }
}