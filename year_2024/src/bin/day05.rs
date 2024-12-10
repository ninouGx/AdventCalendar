use std::str::FromStr;

struct PageOrder {
    before_page: usize,
    after_page: usize,
}

impl FromStr for PageOrder {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nbs: Vec<usize> = s
            .split('|')
            .map(|str| str.parse().unwrap())
            .collect();

        let before_page = nbs[0];
        let after_page = nbs[1];

        Ok(PageOrder { before_page, after_page })
    }
}

struct UpdateSequence {
    list: Vec<usize>,
    size: usize,
}

impl FromIterator<usize> for UpdateSequence {
    fn from_iter<I: IntoIterator<Item = usize>>(iter: I) -> Self {
        let list: Vec<usize> = iter.into_iter().collect();
        let size = list.len();
        UpdateSequence { list, size }
    }
}

fn is_page_in_order(page: usize, sequence: &UpdateSequence, page_orders: &Vec<PageOrder>) -> bool {
    let page_index = sequence.list
        .iter()
        .position(|&p| p == page)
        .expect("The value must exist in the sequence");
    page_orders
        .iter()
        .filter(|page_order| page_order.before_page == page)
        .filter(|page_order| {
            sequence.list.contains(&page_order.after_page) && page_order.after_page != page
        })
        .all(|page_order| {
            let after_page_index = sequence.list
                .iter()
                .position(|&p| p == page_order.after_page)
                .expect("After page must exist in sequence");
            after_page_index > page_index
        })
}

fn is_sequence_in_order(sequence: &UpdateSequence, page_orders: &Vec<PageOrder>) -> bool {
    sequence.list.iter().all(|&page| { is_page_in_order(page, sequence, page_orders) })
}

fn re_order_sequence(sequence: &UpdateSequence, page_orders: &Vec<PageOrder>) -> UpdateSequence {
    let mut list = sequence.list.clone();
    list.sort_by(|&first_page, &second_page| {
        if
            page_orders
                .iter()
                .any(
                    |page_order|
                        page_order.before_page == first_page && page_order.after_page == second_page
                )
        {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    UpdateSequence { list, size: sequence.size }
}

fn middle_page_number_from_sequence(
    sequence: &UpdateSequence,
    page_orders: &Vec<PageOrder>,
    want_ordered: bool
) -> usize {
    match (want_ordered, is_sequence_in_order(sequence, page_orders)) {
        (true, true) => sequence.list[(sequence.size - 1) / 2],
        (false, false) => {
            let re_ordered_sequence = re_order_sequence(sequence, page_orders);
            re_ordered_sequence.list[(re_ordered_sequence.size - 1) / 2]
        }
        _ => 0,
    }
}

fn sum_middle_page(
    updates: &Vec<UpdateSequence>,
    pages: &Vec<PageOrder>,
    want_ordered: bool
) -> usize {
    updates
        .iter()
        .map(|sequence| { middle_page_number_from_sequence(sequence, pages, want_ordered) })
        .sum()
}

fn retrieve_data(input: &str) -> (Vec<PageOrder>, Vec<UpdateSequence>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let pages: Vec<PageOrder> = parts[0]
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let updates: Vec<UpdateSequence> = parts[1]
        .lines()
        .map(|line| {
            let update_sequence: UpdateSequence = line
                .split(',')
                .map(|nb| nb.parse().unwrap())
                .collect();
            update_sequence
        })
        .collect();

    (pages, updates)
}

fn part1(input: &str) -> usize {
    let (pages, updates) = retrieve_data(input);
    sum_middle_page(&updates, &pages, true)
}

fn part2(input: &str) -> usize {
    let (pages, updates) = retrieve_data(input);
    sum_middle_page(&updates, &pages, false)
}

fn main() {
    let is_test = false;

    let input = aoc_utils::get_input_for_day(is_test);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
