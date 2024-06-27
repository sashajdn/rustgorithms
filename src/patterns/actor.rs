use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub fn run_pattern() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let tx = setup_actor();
    let tx2 = tx.clone();

    runtime.block_on(async {
        let jh = tokio::spawn(async move {
            if let Err(e) = tx.send(MonotonicSequenceMessage::IncSequence(10)).await {
                println!("Error sending inc message: {}", e);
            }
        });

        let jh2 = tokio::spawn(async move {
            let (query_tx, query_rx) = oneshot::channel();
            if let Err(e) = tx2
                .send(MonotonicSequenceMessage::QuerySequence(query_tx))
                .await
            {
                println!("Error sending query message: {}", e)
            }

            let count = query_rx.await.unwrap();
            println!("Count: {}", count);
        });

        let (_, _) = tokio::join!(jh, jh2);
    });
}

pub fn setup_actor() -> MonotonicSequenceActorMessageSender {
    let (tx, rx) = mpsc::channel(8);
    let actor = MonotonicSequenceActor::new(rx);

    tokio::spawn(run_monotonic_sequence_actor(actor));

    tx
}

pub struct MonotonicSequenceActor {
    receiver: MonotonicSequenceActorReceiver,
    count: u64,
}

pub async fn run_monotonic_sequence_actor(mut actor: MonotonicSequenceActor) {
    while let Some(msg) = actor.receiver.recv().await {
        match msg {
            MonotonicSequenceMessage::IncSequence(n) => {
                actor.count += n;
            }
            MonotonicSequenceMessage::QuerySequence(tx) => {
                let _ = tx.send(actor.count);
            }
            MonotonicSequenceMessage::Shutdown => break,
        }
    }
}

impl MonotonicSequenceActor {
    pub fn new(receiver: MonotonicSequenceActorReceiver) -> Self {
        Self { receiver, count: 0 }
    }
}

#[derive(Debug)]
pub enum MonotonicSequenceMessage {
    IncSequence(u64),
    QuerySequence(QueryMonotonicSequenceCountSender),
    Shutdown,
}

type MonotonicSequenceActorReceiver = mpsc::Receiver<MonotonicSequenceMessage>;
type MonotonicSequenceActorMessageSender = mpsc::Sender<MonotonicSequenceMessage>;

type QueryMonotonicSequenceReceiver = oneshot::Receiver<u64>;
type QueryMonotonicSequenceCountSender = oneshot::Sender<u64>;
