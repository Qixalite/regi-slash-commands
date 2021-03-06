use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn location() -> InteractionResponse {
    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content: "https://docs.qixalite.com/guides/games/tf2/#regions".to_string(),
        }),
    }
}
