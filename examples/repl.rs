use futures::{SinkExt, StreamExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (mut tx, mut rx) =
        simplex_chat_api::stream::connect("ws://localhost:5225".try_into().expect("valid url"))
            .await
            .unwrap();

    tokio::task::spawn(async move {
        while let Some(response) = rx.next().await {
            dbg!(response);
        }
    });

    let mut index = 0;
    let mut editor = rustyline::DefaultEditor::new().expect("created editor");
    while let Ok(line) = editor.readline(">") {
        tx.send(simplex_chat_api::stream::Request {
            id: index.to_string(),
            command: line,
        })
        .await
        .unwrap();
        index += 1;
    }
}
