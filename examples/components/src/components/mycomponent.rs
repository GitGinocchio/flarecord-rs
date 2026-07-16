use flarecord::{
    models::{ChannelType, SelectMenuType, components::{
        content::{media_gallery::{MediaGallery, MediaGalleryItem}, text_display::TextDisplay}, interactive::{button::{
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
    fn build(&self, root: &mut RootComponent) {
        let button = Button::new()
            .style(ButtonStyle::Success)
            .label("test")
            .build();

        let buttons_action_row = ActionRow::new()
            .button(button)
            .build();

        let media_gallery = MediaGallery::new()
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .add_item(MediaGalleryItem::new("https://google.com"))
            .build();

        let container = Container::new()
            .add(media_gallery);

        let separator = Separator::new()
            .divider(true)
            .spacing(3);

        root.add(container);
        root.add(separator);
        root.add(buttons_action_row);
    }

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}