use flarecord::{models::{SelectMenuType, color::Color, components::{content::{text_display::TextDisplay, thumbnail::Thumbnail}, interactive::{button::{Button, ButtonStyle}, select::Select}, layout::{action_row::{ActionRow, IntoActionRow}, container::Container, section::Section}}}, prelude::*};
use async_trait::async_trait;

use crate::components::mycomponent::MyComponent;

pub struct Hello;

#[async_trait(?Send)]
impl Command for Hello {
    fn name(&self) -> String {
        "hello".into()
    }

    fn description(&self) -> String {
        "Say Hi to someone in chat!".into()
    }

    fn options(&self) -> BotResult<CommandOptions> {
        let user_option = CommandOptionBuilder::user("user", "the user to greet")
            .build()?;
       
        Ok(Some(vec![user_option]))
    }

    async fn execute(&self, interaction: CommandInteraction, _ctx: CommandContext) -> BotResult<CommandResponse> {
        interaction.defer(true).await?;

        let author = interaction.author().ok_or(Error::Generic("Missing author".into()))?;
        let user = interaction.data.get_resolved_user("user");

        let message = match user {
            Some(user) => format!("Hello {0}, {1} greeted you", user.mention(), author.mention()),
            None => format!("Hello {0}!", author.mention())
        };


        let select = Select::role()
            .placeholder("Ciaooo")
            .build();

        let action_row = ActionRow::new()
            .select(select)
            .build();

        let thumbnail = Thumbnail::new("https://google.com")
            .description("Un link a google...");

        let text_display = TextDisplay::new()
            .heading(1, "Ciaooo")
            .paragraph("This is some")
            .bold("text");
        
        let section = Section::new()
            .component(text_display)
            .accessory(thumbnail)
            .build();

        let container = Container::new()
            .accent_color(Color::from_rgb(255, 0, 0))
            .add(action_row)
            .add(section);
        
        let response = CommandResponseBuilder::new()
            // TOFIX: Il problema e' il componente custom che non viene tradotto in un componente valido
            // durante la serializzazione
            //.component(MyComponent)
            .component(container)
            .build();

        interaction.edit(response).await
    }
}