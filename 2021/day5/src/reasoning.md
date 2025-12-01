Reasoning & Design Choices
1. Data Structure: Sparse Grid (HashMap) vs. Dense Grid (Vec<Vec<u32>>)

The Choice: I used a HashMap<Point, u32> to store the grid.
Why: In Advent of Code, grid sizes can sometimes be massive (e.g., coordinates going up to 1,000,000). Allocating a 1000x1000 2D array is fine, but if the coordinates were larger, a dense array would crash your memory. A HashMap is a sparse representation—it only stores memory for points that actually have lines on them.
Benefit: It requires zero pre-calculation of the grid boundaries. I didn't need to scan the input to find max_x and max_y before starting.
2. The "Unified" Line Iterator (The Harder Part)

The Challenge: Writing three separate loops for horizontal, vertical, and diagonal lines is tedious and prone to "off-by-one" errors (e.g., should I loop x from start to end or end to start?).
The Solution: I used the .signum() method.
dx = (x2 - x1).signum() returns -1, 0, or 1.
dy = (y2 - y1).signum() returns -1, 0, or 1.
This allowed me to write one single loop that handles all cases:
```rust
for _ in 0..=steps {
    points.push(Point { x, y });
    x += dx;
    y += dy;
}
```
Reasoning & Design Choices
If it's horizontal, dy is 0. If vertical, dx is 0. If diagonal, both are non-zero. This abstraction made the Part 2 implementation trivial.
Thoughts on the Puzzle
Difficulty: This was a step up from Day 4 in terms of geometry, but simpler in terms of state management. You didn't have to simulate a "game" state (like Bingo), just accumulate static data.

The "Trap": The main trap here is usually coordinate ordering.

Input: 9,4 -> 3,4
Naive Loop: for x in 9..3 -> This loop does nothing in Rust (and many languages) because the range is empty.
You have to handle the direction explicitly (swapping start/end or using the signum step approach I used).
Comparison: I liked this one! It feels very "visual." If you printed the grid for the example input, you could actually see the lines crossing, which is very satisfying. It’s a classic "rasterization" problem—drawing vector lines onto a pixel grid.