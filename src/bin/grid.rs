use std::sync::mpsc::channel;

use art::{
    palette::color::Color,
    shapes::{point::Point, rectangle::Rectangle},
    util::filename::generate_filename,
};
use image::{ImageBuffer, Pixel, RgbImage};
use rand::{thread_rng, Rng};
use threadpool::ThreadPool;

fn main() {
    let (width, height) = (4000.0, 4000.0 * 1.4);

    let bounds = Rectangle {
        position: Point(0.0, 0.0),
        width,
        height,
        color: Some(Color::Hex("#fff")),
        ..Default::default()
    };

    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);

    img.fill(255);

    let inner_bounds = bounds.scale(0.8);
    let mut rects: Vec<Rectangle> = vec![];
    let mut rng = rand::thread_rng();

    let mut x: f64 = inner_bounds.position.0;

    while inner_bounds.x_range().contains(&x) {
        let block_width = rng.gen_range(bounds.width * 0.003..bounds.width * 0.04);
        let mut y = inner_bounds.position.1;

        while inner_bounds.y_range().contains(&y) {
            let block_height = if rng.gen_bool(0.2) {
                inner_bounds.height * rng.gen_range(0.045..0.065)
            } else {
                inner_bounds.height * rng.gen_range(0.002..0.01)
            };

            let rect = Rectangle::new(Point(x, y), block_width, block_height);
            rects.push(rect);
            y += block_height;
        }
        x += block_width;
    }

    let count = rects.len();
    let pool = ThreadPool::new(count);
    let (sender, receiver) = channel::<Vec<Point>>();
    for rect in rects {
        let sender = sender.clone();
        pool.execute(move || {
            let mut thread_rng = thread_rng();
            let mut points: Vec<Point> = vec![];
            let dots = get_dot_count(&rect, bounds.height, 3000);
            for _ in 0..dots {
                let circle = Point(
                    thread_rng.gen_range(rect.x_range()),
                    thread_rng.gen_range(rect.y_range()),
                );

                points.push(circle);
            }
            sender.send(points).expect("error");
        });
    }

    receiver.iter().take(count).for_each(|points| {
        for point in points {
            if inner_bounds.x_range().contains(&point.0) {
                img.put_pixel(
                    point.0 as u32,
                    point.1 as u32,
                    image::Rgba([100, 100, 100, 10]).to_rgb(),
                );
            }
        }
    });

    img.save(generate_filename("grid", "png"))
        .expect("could not save file");
}

fn get_dot_count(rect: &Rectangle, render_height: f64, max_count: usize) -> usize {
    let area_str = format!("{}", rect.area());

    let max_str_len = std::cmp::min(area_str.len(), 4);

    let normalized_area = area_str
        .get(0..max_str_len)
        .unwrap_or("0.0")
        .parse::<f64>()
        .unwrap_or(0.);

    let mut rng = rand::thread_rng();
    let count = (render_height - rect.position.1) * rng.gen_range(2.0..4.0) + normalized_area;

    (count as usize).min(max_count)
}
