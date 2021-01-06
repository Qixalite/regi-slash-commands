mod config;
mod location;
mod maps;
mod subscribe;

use crate::discord::interaction::{
    ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
};

pub(crate) fn handle_command(data: &ApplicationCommandInteractionData) -> InteractionResponse {
    match data.name.as_str() {
        "maps" => maps::maps(),
        "configs" => config::config(),
        "locations" => location::location(),
        "subscribe" => subscribe::subscribe(),
        _ => InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        },
    }
}
