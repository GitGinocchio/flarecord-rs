use std::sync::Arc;

use dynosaur::dynosaur;
use crate::error::BotResult;
use crate::models::modals::context::ModalContext;
use crate::models::modals::interaction::ModalInteraction;

pub mod interaction;
pub mod context;
pub mod data;

pub type ModalType = Arc<DynModal<'static>>;

#[dynosaur(DynModal = dyn(box) Modal)]
pub trait Modal: Send + Sync {
    fn id(&self) -> String;

    fn title(&self) -> String;

    fn components(&self) -> Vec<()>;

    async fn on_submit(&self, interaction: ModalInteraction, ctx: ModalContext) -> BotResult<()>;
}

pub trait IntoModal {
    fn into_modal(self) -> ModalType;
}

impl<M: Modal + 'static> IntoModal for M {
    fn into_modal(self) -> ModalType {
        DynModal::new_arc(self)
    }
}