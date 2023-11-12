use std::fmt::Display;

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
    pub fill: Option<Color>,

    /// The fill rule of this path
    ///
    /// [Docs](https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/fill-rule)
    pub fill_rule: FillRule,
}

#[derive(Debug, Default, Clone, Copy)]
pub enum FillRule {
    #[default]
    EvenOdd,
    NonZero,
}

impl Display for FillRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rule = match self {
            FillRule::EvenOdd => "evenodd",
            FillRule::NonZero => "nonzero",
        };

        write!(f, "{}", rule)
    }
}

impl PathStyle {
    pub fn new() -> PathStyle {
        PathStyle::default()
    }

    pub fn stroke_weight(&mut self, weight: f64) -> PathStyle {
        self.stroke_weight = Some(weight);
        *self
    }

    pub fn stroke(&mut self, color: Option<Color>) -> PathStyle {
        self.stroke = color;
        *self
    }

    pub fn fill(&mut self, color: Option<Color>) -> PathStyle {
        self.fill = color;
        *self
    }

    pub fn fill_rule(&mut self, rule: FillRule) -> PathStyle {
        self.fill_rule = rule;
        *self
    }
}
