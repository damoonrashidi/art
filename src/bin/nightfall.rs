use art::{
    palette::color::Color,
    pointmap::pointmap::Pointmap,
    shapes::{
        circle::Circle, path::Path, path_style::PathStyle, point::Point, rectangle::Rectangle,
        shape::Shape,
    },
    util::{filename::generate_filename, math::weighted_random},
};
use image::{ImageBuffer, RgbImage};
use rand::Rng;

fn main() {
    let bounds = Rectangle::new(Point(0.0, 0.0), 3000.0, 3000.0 * 1.4);
    let mut img: RgbImage = ImageBuffer::new(bounds.width as u32, bounds.height as u32);
    let mut rng = rand::thread_rng();

    imageproc::drawing::draw_filled_rect_mut(
        &mut img,
        bounds.into(),
        Color::HSLa(37, 80.0, 97.0, 1.0).into(),
    );

    let spheres = (0..4)
        .into_iter()
        .map(|_| {
            Circle::new(
                Point(
                    rng.gen_range(bounds.x_range()),
                    rng.gen_range(bounds.y_range()),
                ),
                rng.gen_range(bounds.width * 0.1..bounds.width * 0.4),
            )
        })
        .collect::<Vec<Circle>>();

    let mut points = vec![];
    let mut rng = rand::thread_rng();
    let mut pointmap = Pointmap::new(&bounds, &mut points, 2);

    let rects = (0..4)
        .into_iter()
        .map(|i| {
            Rectangle::new(
                Point(
                    bounds.width * 0.1,
                    bounds.position.1 + (bounds.height * 0.05) + bounds.height * 0.2 * i as f64,
                ),
                bounds.width - (bounds.width * 0.2),
                bounds.height * 0.15,
            )
        })
        .collect::<Vec<Rectangle>>();

    rects.into_iter().for_each(|rect| {
        let density =
            weighted_random(5000.0..10000.0, (rect.position.1 * 2.0).max(5000.0)) as usize;

        for _ in 0..density {
            let mut x = rng.gen_range(rect.x_range());
            let mut y = weighted_random(rect.y_range(), rect.position.1);

            let colliding_sphere = spheres.iter().find(|sphere| sphere.contains(&Point(x, y)));

            if let Some(sphere) = colliding_sphere {
                x += sphere.center().angle_to(&Point(x, y)).cos() * sphere.radius;
                y += sphere.center().angle_to(&Point(x, y)).sin() * sphere.radius;
            }

            let point = Point(x, y);
            let _ = pointmap.add_point(point);
        }
    });

    let style = PathStyle::new()
        .stroke_weight(1.0)
        .stroke(Color::HSLa(37, 80.0, 55.0, 0.1));

    pointmap.points().iter().for_each(|point| {
        let neighbors = pointmap.get_neighbors(point, Some(40.0));
        neighbors.iter().take(10).for_each(|neighbor| {
            let line = &mut vec![**point, **neighbor];
            let path = Path::new(line.clone(), style);
            path.render(&mut img);
        });
    });

    img.save(generate_filename("nightfall", "jpg"))
        .expect("Could not save image");
}
