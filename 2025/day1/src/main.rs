use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Failed to read input.txt");
    let (part1, part2) = solve(&input);
    println!("Part 1 Password: {}", part1);
    println!("Part 2 Password: {}", part2);
}

fn solve(input: &str) -> (i32, i64) {
    let mut dial = 50;
    let mut zero_count_p1 = 0;
    let mut zero_count_p2: i64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let direction = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Failed to parse amount");

        match direction {
            "L" => {
                // Part 2 logic for L
                // Distance to 0 going left is equal to current dial position.
                // If dial is 0, distance is 100 (full circle).
                let dist_to_zero = if dial == 0 { 100 } else { dial };
                
                if amount >= dist_to_zero {
                    zero_count_p2 += 1;
                    let remaining = amount - dist_to_zero;
                    zero_count_p2 += (remaining / 100) as i64;
                }

                dial = (dial - amount).rem_euclid(100);
            }
            "R" => {
                // Part 2 logic for R
                // Distance to 0 going right is 100 - dial.
                // If dial is 0, distance is 100.
                // Actually, simpler math from plan: (current + amount) / 100
                // Let's verify:
                // Start 50, R48 -> 98. (50+48)/100 = 0. Correct.
                // Start 98, R2 -> 0. (98+2)/100 = 1. Correct.
                // Start 0, R100 -> 0. (0+100)/100 = 1. Correct.
                // Start 0, R1 -> 1. (0+1)/100 = 0. Correct.
                // Wait, if start is 0, does it count?
                // "points at 0 ... regardless of whether it happens during a rotation or at the end of one"
                // If we are at 0 and rotate R1, we leave 0 immediately. We don't "point at 0" again.
                // So (0+1)/100 = 0 is correct.
                
                zero_count_p2 += ((dial + amount) / 100) as i64;
                dial = (dial + amount).rem_euclid(100);
            }
            _ => panic!("Unknown direction: {}", direction),
        }

        if dial == 0 {
            zero_count_p1 += 1;
        }
    }
    (zero_count_p1, zero_count_p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let rotations = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        let (p1, p2) = solve(rotations);
        assert_eq!(p1, 3);
        assert_eq!(p2, 6);
    }
}
