use std::fmt::Debug;

use crate::shapes::{point::Point, shape::Shape};

use super::group_style::GroupStyle;

/**
A Group (https://developer.mozilla.org/en-US/docs/Web/SVG/Element/g) can contain other shapes but
ensures that they have the same styles (fill, stroke, stroke_width) applied to them. This can be
advantageous if you have a lot of shapes with the same styles, since the output SVG will be smaller
if the styles are hoisted to the group instead of applied at the shape level.

Example:

```
use art::{
    shapes::{rectangle::Rectangle, point::Point},
    palette::color::Color,
    svg::{group::Group, group_style::GroupStyle}
};

let mut g = Group::new(None);

let rect1 = Rectangle::new(Point(0.0, 0.0), 100.0, 100.0);
let rect2 = Rectangle::new(Point(0.0, 0.0), 100.0, 100.0);

g.add_shape(Box::new(rect1));
g.add_shape(Box::new(rect2));
```
*/
#[derive(Default, Clone)]
pub struct Group {
    shapes: String,
    rotation: Option<f64>,
    rotation_center: Option<Point>,
    style: Option<GroupStyle>,
}

impl Debug for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Group")
    }
}

impl Group {
    pub fn new() -> Group {
        Group {
            shapes: String::from(""),
            rotation: None,
            rotation_center: None,
            style: None,
        }
    }

    /// Add a new shape to the group
    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes = format!("{}{}", self.shapes, shape.as_svg());
    }

    pub fn rotate(&mut self, angle: f64, center: &Point) -> Group {
        Group {
            shapes: self.shapes.clone(),
            rotation: Some(angle),
            rotation_center: Some(*center),
            style: self.style,
        }
    }

    pub fn set_style(&mut self, style: GroupStyle) -> Group {
        Group {
            shapes: self.shapes.clone(),
            rotation: self.rotation,
            rotation_center: self.rotation_center,
            style: Some(style),
        }
    }

    /// Get the entire SVG string of the group
    pub fn as_svg(&self) -> String {
        let style = match self.style {
            Some(ref style) => format!("{}", style),
            None => String::from(""),
        };

        let rotation = match (self.rotation, self.rotation_center) {
            (Some(angle), Some(center)) => format!(
                " transform=\"rotate({})\" transform-origin=\"{} {}\"",
                angle, center.0, center.1
            ),
            _ => String::from(""),
        };

        format!("<g{style}{rotation}>{}</g>", self.shapes)
    }
}

#[cfg(test)]
mod test {

    use crate::{
        palette::color::Color,
        shapes::{point::Point, rectangle::Rectangle},
    };

    use super::Group;

    #[test]
    fn render() {
        let rect = Rectangle::new(Point(0., 0.), 10.0, 10.0);
        let mut g = Group::new().set_style(super::GroupStyle {
            fill: Some(Color::Hex("#111")),
            stroke: None,
            stroke_width: None,
        });

        g.add_shape(Box::new(rect));

        assert_eq!(
            g.as_svg(),
            String::from(
                "<g fill=\"#111\"><rect x=\"0.00\" y=\"0.00\" width=\"10.00\" height=\"10.00\"/></g>"
            )
        );
    }

    #[test]
    fn rotate_group() {
        let mut g = Group::new();

        g.rotate(100.0, &Point(50., 50.));

        assert_eq!(g.as_svg(), "<g transform=\"rotate(100)\"></g>")
    }
}
