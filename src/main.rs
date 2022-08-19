use macroquad::prelude::*;
use quad_net::web_socket::WebSocket;

#[macroquad::main("Networking!")]
async fn main() {
    let mut last_message = String::new();
    let mut first_message = true;
    let mut socket = WebSocket::connect("ws://echo.websocket.events").unwrap();

    loop {
        if socket.connected() {
            if first_message {
                let message = "Hello world";
                info!("Send message {}", message);
                socket.send_text(message);
                first_message = false;
            }
        }

        while let Some(data) = socket.try_recv() {
            last_message = String::from_utf8(data).unwrap();
        }
        draw_text(
            &format!("Last message received: '{}'", last_message),
            50.,
            50.,
            31.0,
            WHITE,
        );

        next_frame().await
    }
}
