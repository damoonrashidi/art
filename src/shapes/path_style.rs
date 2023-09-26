use crate::palette::color::Color;

/// A style for a given [`Path`], it can specify fill, stroke color and stroke width
#[derive(Debug, Default, Clone, Copy)]
pub struct PathStyle {
    /// The width of the stroke around this path
    ///
    /// [Docs](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke-width)
    pub stroke_weight: Option<f64>,

    /// The color of the stroke around this path
    ///
    /// [Docs](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/stroke)
    pub stroke: Option<Color>,

    /// The fill color of this path
    ///
    /// [Docs](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill)
    pub color: Option<Color>,
}

impl PathStyle {
    pub fn new() -> PathStyle {
        PathStyle::default()
    }

    pub fn stroke_weight(&mut self, weight: f64) -> PathStyle {
        self.stroke_weight = Some(weight);
        *self
    }

    pub fn stroke(&mut self, color: Color) -> PathStyle {
        self.stroke = Some(color);
        *self
    }

    pub fn color(&mut self, color: Color) -> PathStyle {
        self.color = Some(color);
        *self
    }
}
