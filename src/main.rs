use cli::Mode;

mod cli;
mod db;
mod increment;
mod sort;
mod wrap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct HistoryItem {
    count: i32,
    value: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HistoryResults(Vec<HistoryItem>);

impl From<Vec<HistoryItem>> for HistoryResults {
    fn from(val: Vec<HistoryItem>) -> Self {
        HistoryResults(val)
    }
}

fn read_lines() -> Vec<String> {
    use std::io::{
        stdin,
        BufRead,
    };
    stdin().lock().lines().flatten().collect()
}

fn main() {
    let matches = cli::get_cli().get_matches();
    let history = db::get_history();

    match matches.subcommand_name().unwrap().into() {
        Mode::Query => {
            let history = &history;
            for line in &history.0 {
                println!("{} {}", line.count, line.value);
            }
        }
        Mode::Sort => {
            let input_lines = read_lines();
            let history = &sort::run(history, input_lines);

            for line in &history.0 {
                println!("{}", line.value);
            }
        }
        Mode::Increment => {
            let input_lines = read_lines();
            db::put_history(increment::run(history, input_lines));
        }
        Mode::Wrap => {
            let input_lines = read_lines();

            let args = matches.subcommand().unwrap().1;

            let cmd = args
                .values_of("command")
                .map(|x| x.collect::<Vec<_>>())
                .unwrap();
            db::put_history(wrap::run(history, input_lines, cmd));
        }
    }
}
