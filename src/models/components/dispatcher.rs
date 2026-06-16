use crate::{error::BotResult, models::{command::response::CommandResponse, components::{ComponentType, context::ComponentContext, interaction::ComponentInteraction}}};


pub (crate) struct ComponentDispatcher;

impl ComponentDispatcher {
    pub (crate) async fn dispatch(
        component: &ComponentType, 
        interaction: ComponentInteraction, 
        ctx: ComponentContext
    ) -> BotResult<CommandResponse> {
        worker::console_debug!("component_id received: {}", interaction.data.custom_id);

        /*
        if component.id() == interaction.data.custom_id {
            return component.handle(interaction, ctx).await;
        }

        let _root = component.build();

        */

        Ok(CommandResponse::new())
    }
}