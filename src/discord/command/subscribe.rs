use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn subscribe() -> InteractionResponse {
    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: "https://docs.qixalite.com/support/subscribers/".to_string(),
        }),
    }
}
