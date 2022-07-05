use clap::Parser;
use image::{ImageBuffer, Luma};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of edges (eg: 3 for a triangle, 4 for a square)
    #[clap(short, long, default_value_t = 5)]
    edges: usize,

    /// Number of points (eg: 1000 puts one thousand black points on the picture)
    #[clap(short, long, default_value_t = 1000000)]
    points: u32,

    /// Radius of the image (eg: 512 results in a 1024 x 1024 pixel image)
    #[clap(short, long, default_value_t = 1024)]
    radius: u32,

    /// Randomizes the starting points
    #[clap(long, short)]
    shuffle: bool,
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone, Copy, Debug)]
struct Pixel {
    x: u32,
    y: u32,
    color: Luma<u8>,
}

impl Point {
    fn to_pixel(self, radius: f64, color: Luma<u8>) -> Pixel {
        Pixel {
            x: (radius - self.x).round() as u32,
            y: (radius - self.y).round() as u32,
            color,
        }
    }

    fn init_points(points: usize, radius: f64) -> Vec<Self> {
        const START_ANGLE: f64 = std::f64::consts::FRAC_PI_2; // Angle starts at pi/2

        (0..points)
            .map(|p| {
                let angle: f64 = START_ANGLE + p as f64 * std::f64::consts::TAU / points as f64;
                Point {
                    x: radius * angle.cos(),
                    y: radius * angle.sin(),
                }
            })
            .collect()
    }

    fn init_random_points(points: usize, radius: f64) -> Vec<Self> {
        (0..points)
            .map(|p| {
                let start_angle = rand::thread_rng().gen_range(0..10000) as f64;
                let angle: f64 = start_angle + p as f64 * std::f64::consts::TAU / points as f64;
                Point {
                    x: radius * angle.cos(),
                    y: radius * angle.sin(),
                }
            })
            .collect()
    }
}

fn main() {
    let args = Args::parse();
    let radius = args.radius as f64;
    assert!(radius >= 2.0, "Radius must be bigger than 1!");

    // Constructs a white image with the dimensions (2 radius x 2 radius)
    let mut img = ImageBuffer::from_fn((radius * 2.0) as u32, (radius * 2.0) as u32, |_, _| {
        Luma([255u8])
    });
    let radius = radius - 1.0;
    let edges = match args.shuffle {
        false => Point::init_points(args.edges, radius),
        true => Point::init_random_points(args.edges, radius),
    };

    // Constructs a random Point
    let mut point = Point {
        x: rand::thread_rng().gen_range((-radius)..radius),
        y: rand::thread_rng().gen_range((-radius)..radius),
    };

    for _ in 0..args.points {
        let edge = edges.choose(&mut rand::thread_rng()).unwrap();
        point = Point {
            x: (edge.x - &point.x) / 2.0,
            y: (edge.y - &point.y) / 2.0,
        };
        let color = match (point.y.round().abs() as u32 * point.x.round().abs() as u32) % 10 {
            0 => 10u8,
            _ => 0u8,
        };

        let pixel = point.to_pixel(radius, Luma([color]));

        img.put_pixel(pixel.x, pixel.y, pixel.color);
    }
    img.save("fractal.png").unwrap();
    println!("Produced fractal.png")
}

