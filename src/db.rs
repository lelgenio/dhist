use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        BufWriter,
        Write,
    },
    path::PathBuf,
};

use dirs::data_dir;

use crate::{
    HistoryItem,
    HistoryResults,
};

fn get_db_path() -> PathBuf {
    if let Ok(histfile_path) = std::env::var("DMENU_HISTORY_FILE") {
        PathBuf::from(histfile_path)
    } else {
        data_dir()
            .expect("Could not get a data dir")
            .join("dmenu-history")
    }
}

fn read_history(file: &File) -> HistoryResults {
    let reader = BufReader::new(file);

    reader
        .lines()
        .flatten()
        .filter_map(|line| -> Option<HistoryItem> {
            let (count, value) = line.split_once(" ")?;

            Some(HistoryItem {
                count: count.parse::<i32>().ok()?,
                value: value.to_string(),
            })
        })
        .collect::<Vec<_>>()
        .into()
}

pub fn get_history() -> HistoryResults {
    let mut hist = match &File::open(get_db_path()) {
        Ok(file) => read_history(file),
        Err(_) => Vec::new().into(),
    };

    hist.0.sort_by_key(|i| -i.count);

    hist
}

pub fn put_history(res: HistoryResults) {
    let file = File::create(get_db_path()).expect("Cannot write to data dir");
    let mut history_writer = BufWriter::new(file);
    for HistoryItem { count, value } in res.0 {
        let newline = format!("{} {}\n", count, value);
        history_writer
            .write_all(newline.as_bytes())
            .expect("Cannot write to history file");
    }
}
