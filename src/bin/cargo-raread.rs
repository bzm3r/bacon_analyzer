use notify::{Config, RecommendedWatcher};
use notify_debouncer_full::{
    new_debouncer,
    notify::{
        event::{AccessKind, AccessMode, CreateKind, DataChange, ModifyKind},
        Event, EventKind, ReadDirectoryChangesWatcher, RecursiveMode, Watcher,
    },
    DebouncedEvent, Debouncer, FileIdCache, FileIdMap,
};
use notify_debouncer_full::{notify, DebounceEventHandler, DebounceEventResult};
use std::{error, sync::mpsc::channel, thread::sleep};
use std::{f32::consts::E, fmt};
use std::{
    fs::OpenOptions,
    io::{self, Write},
    path::Path,
};
use std::{io::Read, sync::mpsc::Receiver, time::Duration};

const JSON_OUT_PATH: &str = "./json.out";

fn main() -> Result<(), RaReadError> {
    let json_result = fetch_json();
    let Err(error) = json_result.and_then(|json| json_to_stdout(json)) else { return Ok(()); };
    Ok(())
}

fn json_to_stdout(json: Vec<u8>) -> Result<(), RaReadError> {
    io::stdout()
        .write_all(&json)
        .map_err(RaReadError::write_err)
}

#[derive(Debug)]
enum RaReadError {
    FileNotFound,
    PermissionDenied,
    EmptyFile,
    Write(io::ErrorKind),
    Other(io::ErrorKind),
    Debouncer(Vec<notify::Error>),
}

impl RaReadError {
    fn read_err(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound => RaReadError::FileNotFound,
            io::ErrorKind::PermissionDenied => RaReadError::PermissionDenied,
            err_kind => RaReadError::Other(err_kind),
        }
    }

    fn write_err(value: io::Error) -> Self {
        RaReadError::Write(value.kind())
    }

    fn could_resolve_by(&self, event: notify::Event) -> bool {
        let ev_kind = event.kind;
        // match ev_kind {
        //     EventKind::Any => todo!(),
        //     EventKind::Access(_) => todo!(),
        //     EventKind::Create(_) => todo!(),
        //     EventKind::Modify(_) => todo!(),
        //     EventKind::Remove(_) => todo!(),
        //     EventKind::Other => todo!(),
        // }
        kind_is_any_of!(event.kind; match self {
            RaReadError::FileNotFound => [Create(File)],
            RaReadError::PermissionDenied => [Access(Close(Any), Other, Any)],
            RaReadError::EmptyFile => [Create(File), Modify(DataChange(_)), Access(Close(Write, Any), Other, Any)],
            RaReadError::Write(_) | RaReadError::Other(_) | RaReadError::Debouncer(_) => false,
        })
    }

    fn matches(&self, events: Vec<notify::Event>) -> bool {
        events.into_iter().any(|event| self.could_resolve_by(event))
    }
}

impl fmt::Display for RaReadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?}", self)
    }
}

impl error::Error for RaReadError {}

fn fetch_json() -> Result<Vec<u8>, RaReadError> {
    let mut f = OpenOptions::new()
        .read(true)
        .open(JSON_OUT_PATH)
        .map_err(RaReadError::read_err)?;
    let mut buf = Vec::new();
    let n = f.read(&mut buf).map_err(RaReadError::read_err)?;
    if n > 0 {
        return Ok(buf);
    } else {
        return Err(RaReadError::EmptyFile);
    }
}

fn debouncer() -> notify::Result<(
    Debouncer<ReadDirectoryChangesWatcher, FileIdMap>,
    Receiver<DebounceEventResult>,
)> {
    let (mut tx, rx) = channel();

    let debouncer = new_debouncer(Duration::from_millis(250), None, tx)?;

    Ok((debouncer, rx))
}

async fn watch_till_fetch(mut fetch_error: RaReadError) -> Vec<u8> {
    let (mut debouncer, mut rx) = debouncer()?;

    debouncer
        .watcher()
        .watch(Path::new(JSON_OUT_PATH), RecursiveMode::Recursive)
        .unwrap();

    loop {
        let events = rx
            .recv_timeout(Duration::from_micros(500))
            .unwrap()
            .unwrap();

        if fetch_error.matches(events) {
            match fetch_json() {
                Ok(data) => {
                    return data;
                }
                Err(err) => fetch_error = err,
            }
        }
    }
}
