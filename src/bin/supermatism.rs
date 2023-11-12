use art::{
    palette::color::Color,
    shapes::{point::Point, rectangle::Rectangle},
    svg::document::Document,
};

#[allow(unused)]
fn main() {
    let bounds =
        Rectangle::new(Point(0., 0.), 1000., 1400.0).set_color(Color::HSLa(35, 65., 97., 1.));
    let mut doc = Document::new("supermatism", bounds);
    let noise = noise::OpenSimplex::new(100);
    let mut rng = rand::thread_rng();

    let inner = bounds.scale(0.9);

    doc.add_shape(Box::new(bounds));

    doc.save();
}
