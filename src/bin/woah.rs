#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn shoelace_area(coordinates: &[Point]) -> f64 {
    let n = coordinates.len();
    let mut sum = 0.0;

    for i in 0..n - 1 {
        sum += coordinates[i].x * coordinates[i + 1].y - coordinates[i + 1].x * coordinates[i].y;
    }

    // Add the contribution of the last edge
    sum += coordinates[n - 1].x * coordinates[0].y - coordinates[0].x * coordinates[n - 1].y;

    // Take the absolute value and divide by 2
    (sum.abs() / 2.0)
}

fn main() {
    let coordinates = vec![
        Point { x: 0.0, y: 6.0 },
        Point { x: 5.0, y: 6.0 },
        Point { x: 5.0, y: 4.0 },
        Point { x: 7.0, y: 4.0 },
        Point { x: 7.0, y: 6.0 },
        Point { x: 9.0, y: 6.0 },
        Point { x: 9.0, y: 1.0 },
        Point { x: 7.0, y: 1.0 },
        Point { x: 7.0, y: 0.0 },
        Point { x: 5.0, y: 0.0 },
        Point { x: 5.0, y: 2.0 },
        Point { x: 2.0, y: 2.0 },
        Point { x: 2.0, y: 0.0 },
        Point { x: 0.0, y: 0.0 },
    ];

    let area = shoelace_area(&coordinates);
    println!("Area: {}", area);
}