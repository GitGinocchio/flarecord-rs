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

        let text_display2 = TextDisplay::new()
            .heading(1, "Ciaooo")
            .paragraph("This is some")
            .bold("text");

        let thumbnail = Thumbnail::new("https://google.com")
            .description("Un link a google...");
        
        let section = Section::new()
            .component(text_display)
            .accessory(button)
            .build();

        let section2 = Section::new()
            .component(text_display2)
            .accessory(thumbnail)
            .build();
        
        root.add(section);
        root.add(section2);

        root
    }

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}