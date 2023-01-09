use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}

#[derive(Debug)]
enum ActorMessage {
    GetUniqueId { respond_to: oneshot::Sender<u32> },
}

#[derive(Clone, Debug)]
struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg).await;
        }
    }

    async fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id);
            }
        }
    }
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = MyActor::new(receiver);
        tokio::spawn(async move { actor.run().await });
        Self { sender }
    }

    async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId { respond_to: send };

        // send message
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

pub async fn run() {
    let handle = MyActorHandle::new();
    let unique_id = handle.get_unique_id().await;
    log::info!("{:?} unique_id: {}", handle, unique_id);
}
