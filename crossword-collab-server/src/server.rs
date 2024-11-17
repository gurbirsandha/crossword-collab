pub mod api_server {
    use std::collections::HashMap;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use futures::{SinkExt, StreamExt, TryFutureExt};
    use warp::{ws::{Message, WebSocket}, Filter};
    use std::net::{SocketAddrV4, Ipv4Addr};
    use tokio::sync::{mpsc, RwLock};
    use tokio_stream::wrappers::UnboundedReceiverStream; //TODO wtf is this

    use crate::db::postgres;

    type Users = Arc<RwLock<HashMap<usize, mpsc::UnboundedSender<Message>>>>;//TODO wtf is this

    static NEXT_USER_ID: AtomicUsize = AtomicUsize::new(1);//TODO wtf is this

    pub async fn start() {
        let users = Users::default();
        // Turn our "state" into a new Filter...
        let users = warp::any().map(move || users.clone());


        let play = warp::path("play")
            // The `ws()` filter will prepare the Websocket handshake.
            .and(warp::ws())
            .and(users)
            .map(|ws: warp::ws::Ws, users| {
                // And then our closure will be called when it completes...
                println!("WOW1");
                ws.on_upgrade(|websocket| user_connected(websocket, users))
            });

        let fetch_board = warp::path!("board" / String)
            .map(|guid: String| fetch_board(guid)
            // TODO: warp reply??
                    
            );

        

        let routes = play;
            
        warp::serve(routes).run(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3030)).await;
        // println!("WOW");
    }

    async fn user_connected(ws: WebSocket, users: Users) {
        println!("in the on upgrade");
        let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);//TODO wtf is this
        let (mut user_ws_tx, mut user_ws_rx) = ws.split();

        // Use an unbounded channel to handle buffering and flushing of messages
        // to the websocket...
        let (tx, rx) = mpsc::unbounded_channel();
        let mut rx = UnboundedReceiverStream::new(rx);
    
        tokio::task::spawn(async move {
            while let Some(message) = rx.next().await {
                user_ws_tx
                    .send(message)
                    .unwrap_or_else(|e| {
                        eprintln!("websocket send error: {}", e);
                    })
                    .await;
            }
        });
    
        // Save the sender in our list of connected users.
        users.write().await.insert(my_id, tx);

        while let Some(result) = user_ws_rx.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={}): {}", my_id, e);
                    break;
                }
            };
            user_message(my_id, msg, &users).await;
        };
        // user_ws_rx stream will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        user_disconnected(my_id, &users).await;
        // rx.forward(tx).map(|result| {
        //     if let Err(e) = result {
        //         eprintln!("websocket error: {:?}", e);
        //     }
        // });

    }

    async fn user_message(my_id: usize, msg: Message, users: &Users) {
        // Skip any non-Text messages...
        let msg = if let Ok(s) = msg.to_str() {
            s
        } else {
            return;
        };

        let new_msg = format!("{}", msg);

        // New message from this user, send it to everyone else (except same uid)...
        for (&uid, tx) in users.read().await.iter() {
            if my_id != uid {
                if let Err(_disconnected) = tx.send(Message::text(new_msg.clone())) {
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }

    async fn user_disconnected(my_id: usize, users: &Users) {
        eprintln!("good bye user: {}", my_id);

        // Stream closed up, so remove from the user list
        users.write().await.remove(&my_id);
    }

    async fn fetch_board(guid: String) {
        let db = postgres::DB::new();

        db.get_board(guid).await;

    }
}