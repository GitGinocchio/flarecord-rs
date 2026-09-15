use flarecord::prelude::*;



pub struct Ticket;


impl Command for Ticket {
    fn name(&self) -> String {
        "ticket".into()
    }

    fn description(&self) -> String {
        "A command that return a modal".into()
    }

    async fn execute(&self, _interaction: CommandInteraction, _ctx: InteractionContext) ->BotResult<CommandResponse> {

        let response = CommandResponse::builder()
            .build();

        Ok(response)
    }
}