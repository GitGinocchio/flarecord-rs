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
    }, embed::Embed}, prelude::*
};


pub struct MyComponent;

impl Component for MyComponent {
    fn build(&self, root: &mut RootComponent) {
        let text_display = TextDisplay::new()
            .heading(1, "Ciaooo")
            .paragraph("This is some")
            .bold("text");

        let button = Button::new()
            .style(ButtonStyle::Primary)
            .label("Test")
            .on_click(async |_int, _ctx| {
                worker::console_debug!("Button clicked!");
                Ok(())
            })
            .build();

        let select = Select::user()
            .max_values(1)
            .min_values(1)
            .required(true)
            .on_select(async |int, _ctx| {
                int.defer(true).await?;

                let text_display = TextDisplay::new()
                    .heading(1, "Ciaooo")
                    .paragraph(format!("Ciaooo {:?}", int.data.values))
                    .bold("text");

                let thumbnail = Thumbnail::new("https://repository-images.githubusercontent.com/1193730554/424d69b9-90e1-4ec3-9c8e-f2671339459a")
                    .description("Un link a google...");

                let button = Button::new()
                    .url("https://github.com/GitGinocchio/Appunti")
                    .label("Github Link")
                    .build();

                let section = Section::new()
                    .accessory(thumbnail)
                    .component(text_display)
                    .build();

                let action_row = ActionRow::new()
                    .button(button)
                    .build();

                let response = CommandResponse::builder()
                    .component(section)
                    .component(action_row)
                    .build();

                int.edit(response).await?;
                Ok(())
            })
            .build();
        
        let section = Section::new()
            .accessory(button)
            .component(text_display)
            .build();

        let action_row = ActionRow::new()
            .select(select)
            .build();
        
        root.add(section);
        root.add(action_row);
    }

    async fn handle(&self, _interaction: ComponentInteraction, _ctx: ComponentContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::new())
    }
}