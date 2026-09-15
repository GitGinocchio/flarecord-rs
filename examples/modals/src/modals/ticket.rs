use flarecord::prelude::*;

pub struct TicketModal {

}

impl Modal for TicketModal {
    fn name(&self) -> String {
        "ticket-modal".into()
    }

    fn description(&self) -> String {
        "a modal used to send a ticket".into()
    }

    async fn on_submit(&self, _interaction: ModalInteraction, _ctx: InteractionContext) -> BotResult<()> {

        Ok(())
    }
}