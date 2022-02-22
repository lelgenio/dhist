use crate::{
    HistoryItem,
    HistoryResults,
};

fn increment_with(main: &mut HistoryResults, rhs: &HistoryResults) {
    for main_item in main.0.iter_mut() {
        for rhs_item in rhs.0.iter() {
            if rhs_item.value == main_item.value {
                main_item.count += rhs_item.count;
            }
        }
    }
}

fn results_from_vec(mut input: Vec<String>) -> HistoryResults {
    input.dedup();
    input
        .into_iter()
        .map(|item| HistoryItem {
            count: 1,
            value: item,
        })
        .collect::<Vec<_>>()
        .into()
}

fn sort_lines(
    history: HistoryResults,
    mut input: HistoryResults,
) -> HistoryResults {
    increment_with(&mut input, &history);

    // Sort such that items with a bigger count are at the start
    input.0.sort_by_key(|i| -i.count);
    input
}

pub fn run(history: HistoryResults, stdin_read: Vec<String>) -> HistoryResults {
    let input = results_from_vec(stdin_read);
    sort_lines(history, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_stays_empty() {
        let input = vec![].into();
        let history = vec![].into();

        let res = sort_lines(input, history);

        assert_eq!(res, vec![].into());
    }

    #[test]
    fn history_stays_no_input() {
        let input = vec![].into();
        let history = vec![
            HistoryItem {
                count: 1,
                value: "sample text".to_string(),
            },
            HistoryItem {
                count: 2,
                value: "sample text".to_string(),
            },
            HistoryItem {
                count: 3,
                value: "sample text".to_string(),
            },
        ]
        .into();

        let res = sort_lines(history, input);

        assert_eq!(res, vec![].into());
    }
    #[test]
    fn history_stays_no_history() {
        let input = vec![
            HistoryItem {
                count: 1,
                value: "sample text".to_string(),
            },
            HistoryItem {
                count: 2,
                value: "sample text".to_string(),
            },
            HistoryItem {
                count: 3,
                value: "sample text".to_string(),
            },
        ]
        .into();
        let history = vec![].into();

        let res = sort_lines(history, input);

        assert_eq!(
            res,
            vec![
                HistoryItem {
                    count: 3,
                    value: "sample text".to_string(),
                },
                HistoryItem {
                    count: 2,
                    value: "sample text".to_string(),
                },
                HistoryItem {
                    count: 1,
                    value: "sample text".to_string(),
                },
            ]
            .into()
        );
    }

    #[test]
    fn count_occurances() {
        let input =
            vec!["one".to_string(), "two".to_string(), "three".to_string()];
        let history = vec![
            HistoryItem {
                count: 1,
                value: "one".to_string(),
            },
            HistoryItem {
                count: 3,
                value: "two".to_string(),
            },
            HistoryItem {
                count: 1,
                value: "three".to_string(),
            },
        ]
        .into();

        let res = sort_lines(history, results_from_vec(input));

        let expected: HistoryResults = vec![
            HistoryItem {
                count: 4,
                value: "two".to_string(),
            },
            HistoryItem {
                count: 2,
                value: "one".to_string(),
            },
            HistoryItem {
                count: 2,
                value: "three".to_string(),
            },
        ]
        .into();

        assert_eq!(res, expected);
    }
}
