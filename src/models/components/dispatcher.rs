use crate::{error::{BotResult, Error}, models::{command::{response::CommandResponse}, components::{Component, ComponentType, context::ComponentContext, interaction::ComponentInteraction, interactive::button::Button, layout::{LayoutComponent, RootComponent, action_row::{ActionRow, ActionRowChild}, container::{Container, ContainerChild}, section::{Section, SectionAccessory}}}}};


pub (crate) struct ComponentDispatcher;

pub (crate) fn get_next_child(id: &str) -> Result<(usize, &str), Error> {
    let (first, other) = match id.split_once(":") {
        Some(result) => result,
        None => (id, "")
    };

    let Some(id) = first.parse::<usize>().ok() else {
        return Err(Error::InvalidInteraction(format!("invalid component id: {}", id)));
    };

    Ok((id, other))
}

#[allow(unused)]
impl ComponentDispatcher {
    pub (crate) async fn dispatch(
        component: &ComponentType, 
        interaction: ComponentInteraction, 
        ctx: ComponentContext
    ) -> BotResult<CommandResponse> {
        let custom_id = interaction.data.custom_id.clone();

        let Some((component_id, path)) = custom_id.split_once(":") else {
            return Err(Error::InvalidInteraction(format!("invalid component id: {}", interaction.data.custom_id)));
        };

        let mut root = RootComponent::new();
        component.build(&mut root);

        root.set_component_id(component_id.to_string());
        root.assign_ids();

        let Ok((layout_id, path)) = get_next_child(path) else {
            return Ok(CommandResponse::new());
        };

        for child in root.children {
            if child.get_id() as usize != layout_id {
                continue
            }

            return ComponentDispatcher::dispatch_layout(&child, interaction, ctx, path).await;
        }

        component.handle(interaction, ctx).await
    }

    pub (crate) async fn dispatch_layout(
        layout: &LayoutComponent,
        interaction: ComponentInteraction, 
        ctx: ComponentContext,
        path: &str
    ) -> BotResult<CommandResponse> {
        match layout {
            LayoutComponent::ActionRow(action_row) => ComponentDispatcher::dispatch_action_row(
                action_row, 
                interaction, 
                ctx, 
                path
            ).await,
            LayoutComponent::Container(container) => ComponentDispatcher::dispatch_container(
                container, 
                interaction, 
                ctx, 
                path
            ).await,
            LayoutComponent::Section(section) => ComponentDispatcher::dispatch_section(
                section, 
                interaction, 
                ctx, 
                path
            ).await,
            _ => Ok(())
        };

        Ok(CommandResponse::empty())
    }

    pub (crate) async fn dispatch_action_row(
        action_row: &ActionRow,
        interaction: ComponentInteraction, 
        ctx: ComponentContext,
        path: &str
    ) -> BotResult<()> {
        let (child_id, path) = get_next_child(&path)?;

        for child in action_row.get_children() {
            match child {
                ActionRowChild::Button(button) => match button {
                    Button::Normal(button) => {
                        let Some(id) = button.inner.custom_id.as_ref() else {
                            return Err(Error::InvalidInteraction(format!("Button has no custom_id")))
                        };

                        if !id.ends_with(&format!("{child_id}")) {
                            continue
                        }

                        return button.clicked(interaction, ctx).await
                    },
                    _ => {}
                },
                ActionRowChild::Select(select) => {
                    if !select.get_custom_id().ends_with(&format!("{child_id}")) {
                        continue
                    }

                    return select.selected(interaction, ctx).await
                },
            }
        }

        Ok(())
    }

    pub (crate) async fn dispatch_container(
        container: &Container,
        interaction: ComponentInteraction, 
        ctx: ComponentContext,
        path: &str
    ) -> BotResult<()> {
        let (child_id, path) = get_next_child(&path)?;

        let Some(child) = container.children.get(child_id) else {
            return Err(Error::InvalidInteraction(format!("action_row child not found!")))?;
        };

        match child {
            ContainerChild::ActionRow(action_row) => ComponentDispatcher::dispatch_action_row(action_row, interaction, ctx, path).await,
            ContainerChild::Section(section) => ComponentDispatcher::dispatch_section(section, interaction, ctx, path).await,
            _ => Ok(())
        }
    }

    pub (crate) async fn dispatch_section(
        section: &Section,
        interaction: ComponentInteraction, 
        ctx: ComponentContext,
        path: &str
    ) -> BotResult<()> {
        let (child_id, path) = get_next_child(&path)?;

        let Some(accessory) = section.get_accessory() else {
            return Err(Error::InvalidInteraction(format!("action_row child not found!")))?;
        };

        match accessory {
            SectionAccessory::Button(button) => match button {
                Button::Normal(button) => {
                    let Some(id) = &button.inner.custom_id else {
                        return Err(Error::InvalidInteraction(format!("Button has no custom_id")))
                    };

                    if !id.ends_with(&format!("{child_id}")) {
                        return Err(Error::InvalidInteraction(format!("Button has an invalid custom_id")))
                    }

                    return button.clicked(interaction, ctx).await
                },
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}