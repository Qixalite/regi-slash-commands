use crate::discord::interaction::{
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn config() -> InteractionResponse {
    InteractionResponse {
        ty: InteractionResponseType::ChannelMessageWithSource,
        data: Some(InteractionApplicationCommandCallbackData {
            content:
                "https://docs.qixalite.com/guides/games/tf2/bk-configs.html#respawn-league-rsl"
                    .to_string(),
        }),
    }
}
