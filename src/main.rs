extern crate clap;
extern crate notify;

use std::any::{Any, TypeId};
use std::sync::mpsc::channel;
use std::time::Duration;

use clap::Clap;
use notify::{DebouncedEvent, RecursiveMode, Result, watcher, Watcher};

#[derive(Clap)]
#[clap(version = "0.0.2")]
struct Opts {
    #[clap(short, long, default_value = "0")]
    verbose: i32,
}

fn is_debounced<T: ?Sized + Any>(_: &T) -> bool {
    TypeId::of::<DebouncedEvent>() == TypeId::of::<T>()
}

fn handle(evt: DebouncedEvent) {
    match evt {
        DebouncedEvent::Create(..) => println!("create"),
        DebouncedEvent::Rename(..) => println!("rename"),
        DebouncedEvent::Remove(..) => println!("remove"),
        DebouncedEvent::Write(path) => println!("write: {:?}", path),
        _ => println!("other"),
    }
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    println!("verbose: {}", opts.verbose);

    let (tx, rx) = channel();
    let mut w = watcher(tx, Duration::from_secs(1))?;

    // TODO
    w.watch(".", RecursiveMode::Recursive)?;

    loop {
        match rx.recv() {
            Ok(evt) if is_debounced(&evt) => handle(evt),
            Ok(evt) => {
                println!("evt: {:?}", evt);
            }
            Err(err) => {
                println!("err: {:?}", err);
                break;
            }
        }
    }

    // user
    let e = notify::Error::Generic("".to_string());
    Err(e)
}

#[cfg(test)]
mod test {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_is_debounced() {
        let mut buf = PathBuf::new();
        buf.push("foo");

        let e = DebouncedEvent::Create(buf);
        assert!(is_debounced(&e));

        struct UnknownEvent;

        let e = UnknownEvent {};
        assert!(!is_debounced(&e));
    }
}
