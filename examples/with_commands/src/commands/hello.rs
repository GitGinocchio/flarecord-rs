use flarecord::{ 
    prelude::*
};
use async_trait::async_trait;

use crate::components::mycomponent::MyComponent;

#[flarecord::command]
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

        let response = CommandResponseBuilder::new()
            .component(MyComponent)
            .build();

        interaction.edit(response).await
    }
}