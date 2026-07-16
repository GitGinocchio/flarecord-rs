use std::sync::{Arc, Mutex};

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
        let layout_handler = LayoutComponentHandler::new(self);
        DynComponent::new_arc(layout_handler)
    }
}

pub (crate) struct LayoutComponentHandler(Mutex<Option<LayoutComponent>>);

impl LayoutComponentHandler {
    pub fn new(layout: LayoutComponent) -> Self {
        Self(Mutex::new(Some(layout)))
    }
}

impl Component for LayoutComponentHandler {
    fn build(&self, root: &mut RootComponent) {
        if let Ok(mut lock) = self.0.lock() {
            if let Some(layout) = lock.take() {
                root.add(layout);
            }
        }
    }

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::empty())
    }
}

/*
pub (crate) struct RootComponentHandler(RwLock<Option<Box<dyn FnOnce(&mut RootComponent) + 'static>>>);

impl RootComponentHandler {
    pub fn new<T>(handler: T) -> Self 
    where 
        T: FnOnce(&mut RootComponent) + 'static 
    {
        Self(RwLock::new(Some(Box::new(handler))))
    }
}

impl Component for RootComponentHandler {
    fn build(&self, root: &mut RootComponent) {
        let handler = {
            let mut lock = self.0.write().expect("RwLock poisoned");
            lock.take()
        };

        if let Some(handler) = handler {
            handler(root);
        } else {
            worker::console_warn!("RootComponentHandler: build already executed.");
        }
    }

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::empty())
    }
}
*/

#[allow(async_fn_in_trait)]
#[dynosaur(DynComponent = dyn(box) Component)]
pub trait Component: Send + Sync {
    fn build(&self, root: &mut RootComponent);

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::empty())
    }
}