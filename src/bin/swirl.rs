use art::{
    palette::palettes,
    pointmap::map::Pointmap,
    shapes::{path::Path, path_style::PathStyle, point::Point, rectangle::Rectangle, shape::Shape},
    svg::document::Document,
};
use rand::Rng;

fn main() {
    let (bg, palette) = palettes::Palettes::orange_autumn();
    let bounds = Rectangle::new(Point(0., 0.), 1000., 1000.0 * 1.4).set_color(bg);
    let mut doc = Document::new("swirl", bounds);
    let mut rng = rand::thread_rng();
    let mut points: Vec<Vec<Point>> = vec![];
    let mut pointmap = Pointmap::new(&bounds, &mut points, 10);

    doc.add_shape(Box::new(bounds));

    for _ in 0..500 {
        let style = PathStyle::new()
            .stroke(Some(palette.get_random_color().unwrap()))
            .stroke_weight(10.0);
        let mut path = Path::new(vec![], style);
        let mut x = rng.gen_range(bounds.x_range());
        let mut y = rng.gen_range(bounds.y_range());

        while bounds.contains(&Point(x, y)) && path.length() < 400. {
            let point = Point(x, y);

            let neighbors = pointmap.get_neighbors(&point, Some(20.));

            if !neighbors.is_empty() {
                break;
            }

            path.add_point(point);
            let n = noise(x, y, &bounds.center()) / 100.;
            x += n.cos() * 2.;
            y += n.sin() * 2.;
        }

        if path.length() < 30. {
            continue;
        }

        for point in &path.points {
            let _ = pointmap.add_point(*point);
        }

        doc.add_shape(Box::new(path));
    }

    doc.save();
}

fn noise(x: f64, y: f64, center: &Point) -> f64 {
    let dist = (x - center.0).hypot(y - center.1);
    let angle = (y - center.1).atan2(x - center.0);

    angle + dist
}
