use super::{point::Point, rectangle::Rectangle};

/// Generic shape definition, can be a Circle, Rectangle, Path, etc
pub trait Shape {
    /// SVG representation of this shape
    fn as_svg(&self) -> String;
    fn render(&self, image: &mut image::RgbImage);

    /// Center Point of this shape
    fn center(&self) -> Point;

    /**
    A tight bounding box around a given shape, this will create a Rectangle around the shape
    */
    fn bounding_box(&self) -> Option<Rectangle>;

    /// True if the given shape contains {point}, otherwise false.
    fn contains(&self, point: &Point) -> bool;
}
