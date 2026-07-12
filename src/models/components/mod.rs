use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use dynosaur::dynosaur;

use crate::models::command::response::CommandResponse;
use crate::models::components::context::ComponentContext;
use crate::models::components::interaction::ComponentInteraction;
use crate::error::BotResult;
use crate::models::components::layout::{LayoutComponent, RootComponent};
use crate::traits::component::IntoComponent;

pub (crate) mod dispatcher;
pub (crate) mod id;
pub mod context;
pub mod interaction;
pub mod content;
pub mod data;

pub mod layout;
pub mod interactive;

pub type ComponentType = Arc<DynComponent<'static>>;

impl<C: Component + 'static> IntoComponent for C {
    fn into_component(self) -> ComponentType {
        DynComponent::new_arc(self)
    }
}

impl IntoComponent for LayoutComponent {
    fn into_component(self) -> ComponentType {
        let mut root = RootComponent::new();
        root.add(self);
        RootComponentHandler::new(root).into_component()
    }
}

pub (crate) struct RootComponentHandler {
    root: Mutex<Option<RootComponent>>,
}

impl RootComponentHandler {
    pub fn new(root: RootComponent) -> Self {
        Self {
            root: Mutex::new(Some(root)),
        }
    }
}

impl Component for RootComponentHandler {
    fn id(&self) -> String {
        "test".into()
    }

    fn build(&self) -> RootComponent {
        self.root
            .lock()
            .expect("Mutex poisoned")
            .take()
            .expect("RootComponent already consumed!")
    }

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::empty())
    }
}

#[dynosaur(DynComponent = dyn(box) Component)]
pub trait Component: Send + Sync {
    fn id(&self) -> String;

    fn build(&self) -> RootComponent;

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse>;
}