use crate::{
    HistoryItem,
    HistoryResults,
};

fn update_history(
    input: Vec<String>,
    history: &HistoryResults,
) -> HistoryResults {
    use std::collections::HashMap;

    let input_lines = input
        .into_iter()
        // Here we copy stdin to stdout so other programs can use it
        .inspect(|i| println!("{}", i))
        .map(|k| HistoryItem { value: k, count: 1 })
        .collect::<Vec<_>>();

    let mut res: HashMap<String, i32> = HashMap::new();

    history.0.iter().chain(input_lines.iter()).for_each(
        |hist_item: &HistoryItem| {
            let entry = res
                .entry(hist_item.value.clone())
                .or_insert(hist_item.count - 1);
            *entry += 1;
        },
    );

    res.into_iter()
        .map(|(k, v)| HistoryItem { value: k, count: v })
        .collect::<Vec<_>>()
        .into()
}

pub fn run(
    history: HistoryResults,
    input_lines: Vec<String>,
) -> HistoryResults {
    // Read

    // Proccess
    update_history(input_lines, &history)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_empty() {
        let input = vec![].into();
        let history = vec![].into();

        update_history(input, &history);

        assert_eq!(history, vec![].into());
    }

    #[test]
    fn increment_keeps_history() {
        let input = vec![].into();
        let history = vec![HistoryItem {
            count: 1,
            value: "one".to_string(),
        }]
        .into();

        update_history(input, &history);

        assert_eq!(
            history,
            vec![HistoryItem {
                count: 1,
                value: "one".to_string(),
            },]
            .into()
        );
    }

    #[test]
    fn increment_empty_history() {
        let input = vec!["one".to_string()].into();
        let history = vec![].into();

        let res = update_history(input, &history);

        assert_eq!(
            res,
            vec![HistoryItem {
                count: 1,
                value: "one".to_string(),
            },]
            .into()
        );
    }

    #[test]
    fn increment_history() {
        let input = vec!["one".to_string()].into();
        let history = vec![HistoryItem {
            count: 1,
            value: "one".to_string(),
        }]
        .into();

        let res = update_history(input, &history);

        assert_eq!(
            res,
            vec![HistoryItem {
                count: 2,
                value: "one".to_string(),
            },]
            .into()
        );
    }
}
