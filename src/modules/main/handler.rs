use axum::extract::{WebSocketUpgrade, ws::WebSocket};

pub async fn ws_handler(ws: WebSocketUpgrade) {
    ws.on_upgrade(move |socket| handler_socket(socket));
}

pub async fn handler_socket(socket: WebSocket) {}
