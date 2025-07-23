use futures::StreamExt;
use simplex_chat_api::{command::Recipient, stream::ResponseData, types::MessageInfo};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (client, mut events, background) =
        simplex_chat_api::ClientBuilder::new("ws://localhost:5225".try_into().expect("valid url"))
            .await
            .unwrap()
            .run();
    tokio::spawn(background);

    while let Some(event) = events.next().await {
        match event.data {
            ResponseData::NewChatItems {
                user: _,
                chat_items,
            } => {
                for item in chat_items {
                    let recipient = match item.info {
                        MessageInfo::Direct { contact } => {
                            Recipient::Direct(contact.local_display_name)
                        }
                        MessageInfo::Group { group_info } => {
                            Recipient::Group(group_info.local_display_name)
                        }
                    };

                    let Ok(number) = item.data.content.content.text().parse::<f64>() else {
                        continue;
                    };

                    let _ = client
                        .say(simplex_chat_api::command::CreateMessage {
                            content: format!("The square of {number} is {}", number * number),
                            recipient,
                        })
                        .await;
                }
            }
            _ => (),
        }
    }
}
