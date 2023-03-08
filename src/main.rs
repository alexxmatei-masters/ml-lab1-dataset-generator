use plotters::prelude::*;
use rand::Rng;
use std::fs::File;
use std::io::{self, prelude::*, BufWriter};

struct Point {
    x: f64,
    y: f64,
    group: u32, // indicates which group the point belongs to
}

// Calculate the probability of a value x in a normal distribution
// with center "m" and dispersion "sigma"
fn gaussian_probability(x: f64, m: f64, sigma: f64) -> f64 {
    let numerator = (m - x).powf(2.0);
    let denominator = 2.0 * sigma.powf(2.0);
    let exponent = -numerator / denominator;
    std::f64::consts::E.powf(exponent)
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
                                                    // print!(
                                                    //     "probability x:{:.2}\n probability thrs: {:.2}\n\n",
                                                    //     probability_x, probability_threshold
                                                    // );
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
                                                    // print!(
                                                    //     "probability y:{:.2}\n probability thrs: {:.2}\n\n",
                                                    //     probability_y, probability_threshold
                                                    // );
        if probability_threshold < probability_y {
            // If the threshold is less than the probability of the generated y-coordinate, accept the y-coordinate
            break;
        }
    }

    Point { x, y, group } // Return the generated point
}

// Generate 10,000 points with coordinates distributed according to normal distributions
// with different means and standard deviations
// The generated points are written to a file named "points.txt"
fn generate_points(filename: &str) -> io::Result<Vec<Point>> {
    // Open the file for writing
    let mut file = BufWriter::new(File::create(filename)?);

    // Define the centers of the Gaussian distributions for each group
    let centers = vec![
        /* mx, my, sigma_x, sigma_y */
        (0.0, 0.0, 10.0, 10.0),
        (-100.0, -100.0, 10.0, 10.0),
        (150.0, -150.0, 10.0, 10.0),
        (50.0, 100.0, 10.0, 10.0),
        (-200.0, 200.0, 10.0, 10.0),
    ];

    let mut points = Vec::new();
    for _ in 0..10000 {
        let mut rng = rand::thread_rng(); // Initialize the random number generator
        let random_group = rng.gen_range(0..=4);
        let mx = centers[random_group].0;
        let my = centers[random_group].1;
        let sigma_x = centers[random_group].2;
        let sigma_y = centers[random_group].3;

        let point = generate_coordinate(random_group as u32, mx, my, sigma_x, sigma_y);

        // Write the point to the file in the format "x y group"
        writeln!(file, "{:.2} {:.2} {}", point.x, point.y, point.group)?;

        // Add the point to the points vector
        points.push(point);
    }

    Ok(points)
}

fn draw_points(points: &Vec<Point>) -> Result<(), Box<dyn std::error::Error>> {
    // Define the dimensions and layout of the plot
    let root = BitMapBackend::new("plot.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let x_min = -300.0;
    let x_max = 300.0;
    let y_min = -300.0;
    let y_max = 300.0;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("Generated Points", ("sans-serif", 50.0))
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    chart.configure_mesh().axis_style(&BLACK).draw()?;

    // Add the points to the chart
    for point in points {
        let color = Palette99::pick(point.group as usize);
        chart.draw_series(std::iter::once(Circle::new(
            (point.x, point.y),
            2,
            color.filled(),
        )))?;
    }

    // Set the legend and finalize the chart
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let points = generate_points("points.txt")?;

    draw_points(&points);
    // // TODO - Debug print
    // for point in &points {
    //     println!("{:.2} {:.2} {}", point.x, point.y, point.group);
    // }

    Ok(())
}
