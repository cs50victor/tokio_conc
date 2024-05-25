use std::time::Duration;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "tokio_conc=debug,ezsockets=debug,livekit=debug,livekit_api=debug,tower_http=debug"
                    .into()
            }),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_file(true)
                .with_line_number(true)
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_target(false)
                .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE),
        )
        .init();

    // these are 'green' threads (thread id changes )
    tokio::spawn(async move {
        loop {
            debug!("run");
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    tokio::spawn(async move {
        loop {
            warn!("ran");
            tokio::time::sleep(Duration::from_secs(3)).await;
        }
    });

    // these are 'actual' OS threads (thread doesn't change )
    std::thread::spawn(|| loop {
        error!("running");
        std::thread::sleep(Duration::from_secs(3));
    });

    // Cell is an acceptable complication when accessing the data.
    let val = std::cell::Cell::new(1);
    // tokio::select! {
    //   _ = async {loop {
    //     info!("1. {}", val.get());
    //     tokio::time::sleep(Duration::from_millis(200)).await;
    //   }} => {},
    //   _ = async {loop {
    //     let x = val.get();
    //     info!("2. {x}");
    //     // The problem: During this await the dots are not printed.
    //     tokio::time::sleep(Duration::from_secs(1)).await;
    //     val.set(val.get() + 1);
    //     tokio::time::sleep(Duration::from_secs(3)).await;
    //   }} => {},
    // }
    tokio::join!(
        async {
            loop {
                info!("1. {}", val.get());
                tokio::time::sleep(Duration::from_millis(1000)).await;
            }
        },
        async {
            loop {
                info!("2. {}", val.get());
                val.set(val.get() + 1);
                tokio::time::sleep(Duration::from_millis(50)).await;
            }
        },
        example()
    );
}

#[tracing::instrument]
fn compute_heavy() -> &'static str {
    let mut i = 0;
    loop {
        if i == 1000 {
            break
        }
        i+=1;
    }
    ""
}

async fn example(){
  loop{
    info!("hello world");
    compute_heavy();
    tokio::time::sleep(Duration::from_millis(1000)).await;
  }
}