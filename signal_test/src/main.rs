use tokio::signal::{self, unix::SignalKind};
use tokio::signal::unix::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let ctrl_c_ft = listen_ctrl_c();
    let term_ft = listen_term();
    let _ = tokio::join!(ctrl_c_ft, term_ft);

    Ok(())
}

async fn listen_ctrl_c() -> Result<(), Box<dyn std::error::Error>> {
    signal::ctrl_c().await?;
    println!("ctrl-c received!");
    signal::ctrl_c().await?;
    println!("ctrl-c received 1!");
    signal::ctrl_c().await?;
    println!("ctrl-c received 2!");
    signal::ctrl_c().await?;
    println!("ctrl-c received 3!");

    Ok(())
}


async fn listen_term() -> Result<(), Box<dyn std::error::Error>> {
    // An infinite stream of hangup signals.
    let mut stream = signal(SignalKind::terminate())?;

    // Print whenever a HUP signal is received
    loop {
        stream.recv().await;
        println!("got signal HUP");
    }
}