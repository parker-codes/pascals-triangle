use factorial::Factorial;
use std::{env, fmt};

fn main() {
    let args: Vec<String> = env::args().collect();
    let number_of_rows: u128 = args[1]
        .parse::<u128>()
        .expect("Couldn't understand how many rows to print");

    println!("{}", Triangle::new(number_of_rows));
}

struct Triangle(Vec<Row>);

impl Triangle {
    fn new(rows: u128) -> Self {
        let items = (0..rows)
            .into_iter()
            .map(|index| Row::new(index))
            .collect::<Vec<_>>();

        Self(items)
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let row_count = self.0.len();

        let rows = self
            .0
            .iter()
            .map(|row| {
                let row = row.to_string();
                return format!("{: ^1$}", row, row_count * 3);
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", rows)
    }
}

struct Row(Vec<u128>);

impl Row {
    fn new(row: u128) -> Self {
        let items = (0..=row)
            .into_iter()
            .map(|index| Row::value_at(row, index))
            .collect::<Vec<_>>();

        Self(items)
    }

    fn value_at(row: u128, column: u128) -> u128 {
        row.factorial() / (column.factorial() * (row - column).factorial())
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let items = self
            .0
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join("  ");

        write!(f, "{}", items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn it_gets_specific_item() {
        assert_eq!(Row::value_at(0, 0), 1);
        assert_eq!(Row::value_at(2, 0), 1);
        assert_eq!(Row::value_at(3, 2), 3);
    }

    #[test]
    fn it_computes_a_specific_row() {
        assert_eq!(Row::new(3).0, vec![1, 3, 3, 1]);
    }

    #[test]
    fn it_builds_triangle() {
        let triangle_output = format!("{}", Triangle::new(3));
        let answer = indoc! {"
            1
            1  1
            1  2  1"};

        assert_eq!(triangle_output, answer);
    }
}
