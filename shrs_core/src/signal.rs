use std::thread;

use crossbeam_channel::{bounded, Receiver};
use signal_hook::{consts::*, iterator::Signals};

pub fn sig_handler() -> anyhow::Result<Receiver<()>> {
    thread::spawn(move || {
        let mut signals = Signals::new(TERM_SIGNALS).unwrap();
        loop {
            for signal in signals.pending() {
                match signal {
                    SIGINT => {
                        println!("\nSIGINT received");
                    },
                    SIGTERM => {
                        println!("\nSIGTERM received");
                    },
                    _ => {},
                }
            }
        }
    });

    let (sender, receiver) = bounded(100);
    Ok(receiver)
}
