use std::fs;


#[derive(Debug, Clone)]
struct Board {
    grid: Vec<Vec<(u32, bool)>>, // (number, marked)
}

impl Board {
    fn new(numbers: Vec<Vec<u32>>) -> Self {
        let grid = numbers
            .into_iter()
            .map(|row| row.into_iter().map(|n| (n, false)).collect())
            .collect();
        Board { grid }
    }

    fn mark(&mut self, number: u32) {
        for row in &mut self.grid {
            for (n, marked) in row {
                if *n == number {
                    *marked = true;
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        // Check rows
        for row in &self.grid {
            if row.iter().all(|(_, marked)| *marked) {
                return true;
            }
        }

        // Check columns
        for col in 0..5 {
            if (0..5).all(|row| self.grid[row][col].1) {
                return true;
            }
        }

        false
    }

    fn score(&self, last_called: u32) -> u32 {
        let sum_unmarked: u32 = self.grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|(_, marked)| !*marked)
            .map(|(n, _)| n)
            .sum();
        sum_unmarked * last_called
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut sections = input.split("\n\n");
    
    let numbers_str = sections.next().unwrap();
    let numbers: Vec<u32> = numbers_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    let boards: Vec<Board> = sections
        .map(|section| {
            let grid: Vec<Vec<u32>> = section
                .lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| {
                    line.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect()
                })
                .collect();
            Board::new(grid)
        })
        .collect();

    (numbers, boards)
}

fn main() {
    let input = fs::read_to_string("part1_input.txt").expect("Failed to read input file");
    let (numbers, boards) = parse_input(&input);

    println!("Loaded {} numbers and {} boards", numbers.len(), boards.len());

    solve_part1(&numbers, boards.clone());
    solve_part2(&numbers, boards);
}

fn solve_part1(numbers: &[u32], mut boards: Vec<Board>) {
    println!("\n--- Part 1 ---");
    for &number in numbers {
        for board in &mut boards {
            board.mark(number);
            if board.check_win() {
                let score = board.score(number);
                println!("First winning board found!");
                println!("Score: {}", score);
                return;
            }
        }
    }
    println!("No winner found for Part 1.");
}

fn solve_part2(numbers: &[u32], mut boards: Vec<Board>) {
    println!("\n--- Part 2 ---");
    let mut won_boards = std::collections::HashSet::new();
    let total_boards = boards.len();
    let mut last_score = 0;

    for &number in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if won_boards.contains(&i) {
                continue;
            }

            board.mark(number);
            if board.check_win() {
                won_boards.insert(i);
                if won_boards.len() == total_boards {
                    let score = board.score(number);
                    println!("Last winning board found (Board {})!", i);
                    println!("Score: {}", score);
                    return;
                }
                last_score = board.score(number); // Keep track just in case
            }
        }
    }
    println!("Game ended. Last winning score recorded: {}", last_score);
}
