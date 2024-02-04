use eframe::egui;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

struct MyApp {
    text: String,
    is_server: bool,
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Egui Text Edit Box");
            ui.label("Type something in the box:");
            let mut output = ui.text_edit_multiline(&mut self.text);
            ui.add_space(10.0); // Add some space between elements
            ui.label(format!("Current text: {}", self.text));

            ui.horizontal(|ui| {
                ui.label("Move cursor to the:");
    
                if ui.button("start").clicked() {
                    let text_edit_id = output.id;
                    if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), text_edit_id) {
                        // let ccursor = egui::text::CCursor::new(0);
                        let mut ccursor_range = egui::text::CCursorRange::default();
                        ccursor_range.primary = egui::text::CCursor::new(0);
                        ccursor_range.secondary = egui::text::CCursor::new(5);
                        // ccursor.
                        state.set_ccursor_range(Some(ccursor_range));
                            // .ccursor_range.primary = 0;
                            // .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
                        state.store(ui.ctx(), text_edit_id);
                        ui.ctx().memory_mut(|mem    | mem.request_focus(text_edit_id)); // give focus back to the [`TextEdit`].
                    }
                }
    
            });

        });
    }
}

async fn run_server(addr: SocketAddr) {
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: TcpStream) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during WebSocket handshake");

    let (mut write, mut read) = ws_stream.split();

    while let Some(Ok(message)) = read.next().await {
        if message.is_text() || message.is_binary() {
            write.send(message).await.expect("Failed to send message");
        }
    }
}

async fn run_client(url: String) {
    let (mut socket, _) = tokio_tungstenite::connect_async(url)
        .await
        .expect("Failed to connect");

    socket.send(Message::Text("Hello WebSocket".into())).await.unwrap();
    while let Some(Ok(message)) = socket.next().await {
        println!("Received a message: {}", message);
    }
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: [server|client]");
        return;
    }

    let is_server = args[1] == "server";

    if is_server {
        // Launch the WebSocket server in a new thread or async runtime
        tokio::spawn(async {
            run_server("127.0.0.1:8080".parse().unwrap()).await;
        });
    } else {
        // Launch the WebSocket client in a new thread or async runtime
        tokio::spawn(async {
            run_client("ws://127.0.0.1:8080".to_string()).await;
        });
    }

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "WebSocket Egui Example",
        options,
        Box::new(move |_| Box::new(MyApp { text: String::new(), is_server })),
    );
}
