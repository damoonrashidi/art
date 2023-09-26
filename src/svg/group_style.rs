use std::fmt::Display;

use crate::palette::color::Color;

/// A group style defined the fill, stroke width and stroke color
/// for all shapes contained in the group, unless the styles are
/// defined on the shape level, in which case they override the
/// group styles.
#[derive(Debug, Default, Copy, Clone)]
pub struct GroupStyle {
    /// Fill color
    pub fill: Option<Color>,

    /// Stroke outline color
    pub stroke: Option<Color>,

    /// Stroke outline width
    pub stroke_width: Option<f64>,
}

impl Display for GroupStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stroke_width: String = match &self.stroke_width {
            Some(width) => format!(" stroke-width=\"{width}\""),
            None => String::from(""),
        };

        let stroke: String = match &self.stroke {
            Some(color) => format!(" stroke=\"{color}\""),
            None => String::from(""),
        };

        let fill: String = match &self.fill {
            Some(color) => format!(" fill=\"{color}\""),
            None => String::from(" fill=\"none\""),
        };

        write!(f, "{fill}{stroke}{stroke_width}")
    }
}
