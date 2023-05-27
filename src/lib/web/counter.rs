use crate::data::DatabasePool;
use crate::domain::clip::field::Shortcode;
use crate::service::{self, ServiceError};
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Receiver, Sender};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;

type HitStore = Arc<Mutex<HashMap<Shortcode, u32>>>;

#[derive(Debug, thiserror::Error)]
enum HitCounterError {
    #[error("Service error: {0}")]
    Service(#[from] ServiceError),
    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCounterMsg>),
}

enum HitCounterMsg {
    Commit,
    Hit(Shortcode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCounterMsg>,
}

impl HitCounter {
    fn commit_hits(
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCounterError> {
        let hits = Arc::clone(&hits);
        let hits: Vec<(Shortcode, u32)> = {
            let mut hits = hits.lock();
            let hits_vec = hits.iter().map(|(k, v)| (k.clone(), *v)).collect();
            hits.clear();
            hits_vec
        };
        handle.block_on(async move {
            let transaction = service::action::begin_transaction(&pool).await?;
            for (shortcode, hits) in hits {
                if let Err(e) = service::action::increase_hits_count(&shortcode, hits, &pool).await
                {
                    eprintln!("error increasing hit count {}", e)
                }
            }
            Ok(service::action::end_transaction(transaction).await?)
        })
    }

    fn process_msg(
        msg: HitCounterMsg,
        hits: HitStore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCounterError> {
        match msg {
            HitCounterMsg::Commit => Self::commit_hits(hits, handle, pool)?,
            HitCounterMsg::Hit(shortcode, count) => {
                let mut hitcount = hits.lock();
                let hitcount = hitcount.entry(shortcode).or_insert(0);
                *hitcount += count;
            }
        }
        Ok(())
    }

    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = unbounded();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let store: HitStore = Arc::new(Mutex::new(HashMap::new()));

            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => {
                        if let Err(e) =
                            Self::process_msg(msg, store.clone(), handle.clone(), pool.clone())
                        {
                            eprintln!("message processing error: {}", e);
                        }
                    }
                    Err(e) => match e {
                        TryRecvError::Empty => {
                            std::thread::sleep(Duration::from_secs(5));
                            if let Err(e) = tx_clone.send(HitCounterMsg::Commit) {
                                eprintln!("error sending commit message to hits channel")
                            }
                        }
                        _ => break,
                    },
                }
            }
        });
        Self { tx }
    }

    pub fn hit(&self, shortcode: Shortcode, count: u32) {
        if let Err(e) = self.tx.send(HitCounterMsg::Hit(shortcode, count)) {
            eprintln!("hit counter: {}", e)
        }
    }
}
