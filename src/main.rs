use rand::Rng;
use std::fs::File;
use std::io::{self, prelude::*, BufWriter};

struct Point {
    x: f64,
    y: f64,
    group: u32, // indicates which group the point belongs to
}

// Calculate the probability of a value x in a normal distribution
// with center "center" and dispersion "dispersion"
fn gaussian_probability(x: f64, center: f64, dispersion: f64) -> f64 {
    let exponent = -((center - x).powi(2)) / (2.0 * dispersion.powi(2));
    let denominator = (2.0 * std::f64::consts::PI * dispersion.powi(2)).sqrt();
    exponent.exp() / denominator
}

// Generate a single point with coordinates (x, y) belonging to the given group
// The coordinates are generated using normal distributions with the given means and standard deviations
fn generate_coordinate(
    group: u32,   // indicates which group the point belongs to
    mx: f64,      // mean of the normal distribution for x-coordinate
    my: f64,      // mean of the normal distribution for y-coordinate
    sigma_x: f64, // standard deviation of the normal distribution for x-coordinate
    sigma_y: f64, // standard deviation of the normal distribution for y-coordinate
) -> Point {
    let mut rng = rand::thread_rng(); // Initialize the random number generator
    let mut x;
    let mut y;

    // Generate the x-coordinate for the point
    loop {
        x = rng.gen_range(-300.0..300.0); // Generate a random value in the range [-300.0, 300.0)
        let probability_x = gaussian_probability(x, mx, sigma_x); // Calculate the probability of the generated x-coordinate
        let probability_threshold: f64 = rng.gen(); // Generate a random probability threshold
        if probability_threshold < probability_x {
            // If the threshold is less than the probability of the generated x-coordinate, accept the x-coordinate
            break;
        }
    }

    // Generate the y-coordinate for the point
    loop {
        y = rng.gen_range(-300.0..300.0); // Generate a random value in the range [-300.0, 300.0)
        let probability_y = gaussian_probability(y, my, sigma_y); // Calculate the probability of the generated y-coordinate
        let probability_threshold: f64 = rng.gen(); // Generate a random probability threshold
        if probability_threshold < probability_y {
            // If the threshold is less than the probability of the generated y-coordinate, accept the y-coordinate
            break;
        }
    }

    Point { x, y, group } // Return the generated point
}

// Generate 50,000 points (5 groups of 10,000 points each) with coordinates distributed according to normal distributions
// with different means and standard deviations
// The generated points are written to a file named "points.txt"
fn generate_points(filename: &str) -> io::Result<Vec<Point>> {
    // Open the file for writing
    let mut file = BufWriter::new(File::create(filename)?);

    // Define the centers of the Gaussian distributions for each group
    let centers = vec![
        /* mx, my, sigma_x, sigma_y */
        (0.0, 0.0, 50.0, 50.0),
        (-100.0, -100.0, 30.0, 60.0),
        (150.0, -150.0, 80.0, 20.0),
        (50.0, 100.0, 70.0, 70.0),
        (-200.0, 200.0, 50.0, 70.0),
    ];

    let mut points = Vec::new();

    // For each group, generate 10,000 points using the generate_coordinate function
    for (_i, (mx, my, sigma_x, sigma_y)) in centers.iter().enumerate() {
        for _ in 0..2000 {
            let mut rng = rand::thread_rng(); // Initialize the random number generator
            let random_group = rng.gen_range(0..=4);
            let point = generate_coordinate(random_group as u32, *mx, *my, *sigma_x, *sigma_y);

            // Write the point to the file in the format "x y group"
            writeln!(file, "{:.2} {:.2} {}", point.x, point.y, point.group)?;

            // Add the point to the points vector
            points.push(point);
        }
    }

    Ok(points)
}

fn main() -> io::Result<()> {
    let points = generate_points("points.txt")?;

    // TODO - Debug print
    for point in &points {
        println!("{:.2} {:.2} {}", point.x, point.y, point.group);
    }

    Ok(())
}
