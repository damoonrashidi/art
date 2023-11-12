use art::{
    palette::color::Color,
    shapes::{
        path::Path,
        path_style::{FillRule, PathStyle},
        point::Point,
        rectangle::Rectangle,
    },
    svg::document::Document,
};
use noise::NoiseFn;
use rand::Rng;

fn main() {
    let (width, height) = (4000.0, 4000.0 * 1.4);

    let bounds = Rectangle {
        position: Point(0.0, 0.0),
        width,
        height,
        color: Some(Color::Hex("#fff")),
        ..Default::default()
    };

    let mut doc = Document::new("dance", bounds);
    let mut rng = rand::thread_rng();
    let noise = noise::OpenSimplex::new(rng.gen_range(0..100));
    let line_2_noise = noise::OpenSimplex::new(rng.gen_range(0..100));

    let mut points = vec![];

    let mut y = 0.;
    let mut x = rng.gen_range(bounds.x_range());
    let start_x = x;
    while y < bounds.height {
        let n = noise.get([y / 1800., x / 1800.]);
        x += (n * 2.2).sin() * 1.5;
        y += 1.5;
        points.push(Point(x, y));
    }
    let path = Path::new(
        points,
        PathStyle::new().fill(None).stroke(Some(Color::Hex("#333"))),
    );

    y = 0.;
    x = start_x + rng.gen_range(-100.0..100.0);
    let mut second_line = vec![];

    while y < bounds.height {
        let n = line_2_noise.get([y / 1800., x / 1800.]);
        x += (n * 2.2).sin() * 1.5;
        y += 1.5;
        second_line.push(Point(x, y));
    }

    let second_path = Path::new(
        second_line,
        PathStyle::new().fill(None).stroke(Some(Color::Hex("#333"))),
    );

    let mut joint_line = vec![];

    joint_line.extend(path.points.clone());
    joint_line.push(*second_path.points.last().unwrap());
    joint_line.extend(second_path.points);
    joint_line.push(*path.points.first().unwrap());

    let joint = Path::new(
        joint_line,
        PathStyle::new()
            .fill(Some(Color::Hex("#baffad")))
            .fill_rule(FillRule::NonZero),
    );

    doc.add_shape(Box::new(joint));

    doc.save();
}
