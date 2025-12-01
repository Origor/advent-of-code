use std::collections::HashMap;
use std::fs;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_orthogonal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();
        let x1 = self.start.x as i32;
        let y1 = self.start.y as i32;
        let x2 = self.end.x as i32;
        let y2 = self.end.y as i32;

        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();

        let steps = std::cmp::max((x2 - x1).abs(), (y2 - y1).abs());

        let mut x = x1;
        let mut y = y1;

        for _ in 0..=steps {
            points.push(Point { x: x as u32, y: y as u32 });
            x += dx;
            y += dy;
        }
        points
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let start_coords: Vec<u32> = parts[0]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            let end_coords: Vec<u32> = parts[1]
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();

            Line {
                start: Point {
                    x: start_coords[0],
                    y: start_coords[1],
                },
                end: Point {
                    x: end_coords[0],
                    y: end_coords[1],
                },
            }
        })
        .collect()
}

fn count_overlaps<'a>(lines: impl Iterator<Item = &'a Line>) -> usize {
    let mut counts: HashMap<Point, u32> = HashMap::new();
    for line in lines {
        for point in line.points() {
            *counts.entry(point).or_insert(0) += 1;
        }
    }
    counts.values().filter(|&&count| count >= 2).count()
}

fn solve_part1(lines: &[Line]) {
    println!("--- Part 1 ---");
    let orthogonal_lines = lines.iter().filter(|l| l.is_orthogonal());
    let overlaps = count_overlaps(orthogonal_lines);
    println!("Overlaps: {}", overlaps);
}

fn solve_part2(lines: &[Line]) {
    println!("--- Part 2 ---");
    let overlaps = count_overlaps(lines.iter());
    println!("Overlaps: {}", overlaps);
}

fn main() {
    let input = fs::read_to_string("part1_input.txt").expect("Failed to read input file");
    let lines = parse_input(&input);

    println!("Loaded {} lines", lines.len());
    solve_part1(&lines);
    solve_part2(&lines);
}
