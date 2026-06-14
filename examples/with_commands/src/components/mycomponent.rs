use async_trait::async_trait;
use flarecord::{
    models::{ChannelType, SelectMenuType, components::{
        content::{media_gallery::{MediaGallery, MediaGalleryItem}, text_display::TextDisplay, thumbnail::Thumbnail}, interactive::{button::{
            Button, 
            ButtonStyle
        }, select::Select}, layout::{
            action_row::{
                ActionRow, 
                IntoActionRow
            }, 
            container::Container, 
            section::Section, 
            separator::Separator
        }
    }}, 
    prelude::*
};


pub struct MyComponent;

#[async_trait(?Send)]
impl Component for MyComponent {
    fn id(&self) -> String {
        "mycomponent".into()
    }

    fn build(&self) -> RootComponent {
        let mut root = RootComponent::new();

        let text_display = TextDisplay::new()
            .heading(1, "Ciaooo")
            .paragraph("This is some")
            .bold("text");

        let button = Button::new()
            .style(ButtonStyle::Primary)
            .label("Test")
            .build();

        let thumbnail = Thumbnail::new("https://google.com")
            .description("Un link a google...");
        
        let section = Section::new()
            .component(text_display)
            .accessory(thumbnail)
            .build();
        
        root.add(section);

        root
    }

    async fn handle(&self, interaction: ComponentInteraction, ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}

/*
command_response: {
    "data": {
        "components": [
            {
                "components": [
                    {
                        "custom_id": String("0"), 
                        "disabled": Bool(false), 
                        "placeholder": String("Ciaooo"), 
                        "type": Number(6)
                    }
                ], 
                "id": Number(0), 
                "type": Number(1)
            }, 
            {
                "accessory":  {
                    "custom_id": String("mycomponent:1"), 
                    "label": String("Test"), 
                    "style": Number(1), 
                    "type": Number(2)
                }, 
                "components": [
                    {
                        "content": String("# Ciaooo\n"), 
                        "id": Number(1), 
                        "type": Number(10)
                    }
                ],
                "id": Number(1), 
                "type": Number(9)
            }
        ], 
        "content": String("Hello <@778017089230209045>!"), 
        "flags": Number(32832)
    }, 
    
    "type": Number(4)
}
*/