use super::{
    path_style::{FillRule, PathStyle},
    point::Point,
    rectangle::Rectangle,
    shape::Shape,
};

/// An SVG path
#[derive(Debug, Clone)]
pub struct Path {
    /// List of points that make up the path.
    pub points: Vec<Point>,

    /// Stroke width, stroke color and fill color.
    pub style: PathStyle,

    rotation: Option<f64>,
    rotation_center: Option<Point>,
}

impl Path {
    /// Create new [`Path`] with the given [`Point`]s and [`PathStyle`]
    pub fn new(points: Vec<Point>, style: PathStyle) -> Path {
        Path {
            points,
            style,
            rotation: None,
            rotation_center: None,
        }
    }

    /// Adds another [`Point`] to the end of this path. This is good if
    /// You want to make a line longer.
    pub fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    pub fn rotate(&mut self, angle: f64, center: Point) -> &Path {
        self.rotation = Some(angle);
        self.rotation_center = Some(center);
        self
    }

    /// The total distance between each point in this shape, i.e, the true
    /// length of the shape.
    pub fn length(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }

        let mut total = 0.0;
        for i in 1..self.points.len() {
            total += self.points[i - 1].distance_to(&self.points[i])
        }
        total
    }

    /// Check if two lines intersect at any point.
    fn intersects(a: (&Point, &Point), b: (&Point, &Point)) -> bool {
        let dx0 = a.1 .0 - a.0 .0;
        let dx1 = b.1 .0 - b.0 .0;
        let dy0 = a.1 .1 - a.0 .1;
        let dy1 = b.1 .1 - b.0 .1;
        let p0 = dy1 * (b.1 .0 - a.0 .0) - dx1 * (b.1 .1 - a.0 .1);
        let p1 = dy1 * (b.1 .0 - a.1 .0) - dx1 * (b.1 .1 - a.1 .1);
        let p2 = dy0 * (a.1 .0 - b.0 .0) - dx0 * (a.1 .1 - b.0 .1);
        let p3 = dy0 * (a.1 .0 - b.1 .0) - dx0 * (a.1 .1 - b.1 .1);
        (p0 * p1 <= 0.0) & (p2 * p3 <= 0.0)
    }
}

impl Shape for Path {
    fn as_svg(&self) -> String {
        if self.points.is_empty() {
            return "".to_string();
        }

        let stroke: String = match self.style.stroke {
            Some(color) => format!("stroke=\"{color}\" "),
            None => "".to_string(),
        };

        let fill: String = match self.style.fill {
            Some(color) => format!("fill=\"{color}\" "),
            None => "fill=\"none\" ".to_string(),
        };

        let stroke_weight: String = match self.style.stroke_weight {
            Some(stroke) => format!("stroke-width=\"{:.2}\" ", stroke),
            None => "".to_string(),
        };

        let fill_rule: String = match self.style.fill_rule {
            FillRule::EvenOdd => String::from(""),
            FillRule::NonZero => String::from("fill-rule=\"nonzero\" "),
        };

        if let Some(first) = self.points.first() {
            let mut str = self.points.iter().skip(1).enumerate().fold(
                format!(
                    "<path {fill}{fill_rule}{stroke}{stroke_weight}d=\"M{:.2},{:.2}",
                    first.0, first.1
                ),
                |mut path, (i, point)| {
                    if let Some(previous) = self.points.get(i) {
                        if previous.0 == point.0 {
                            path.push_str(&format!(" V{:.2}", point.1));
                        } else if previous.1 == point.1 {
                            path.push_str(&format!(" H{:.2}", point.0));
                        } else {
                            path.push_str(&format!(" L{:.2},{:.2}", point.0, point.1));
                        }
                    }

                    path
                },
            );

            str.push_str("\"/>\n");
            return str;
        }
        String::from("")
    }

    fn render(&self, image: &mut image::RgbImage) {
        let color = match self.style.stroke {
            Some(c) => c.into(),
            None => image::Rgb([0, 0, 0]),
        };

        for i in 0..self.points.len() - 1 {
            let start = self.points.get(i).unwrap();
            let end = self.points.get(i + 1).unwrap_or(start);

            imageproc::drawing::draw_line_segment_mut(
                image,
                (start.0 as f32, start.1 as f32),
                (end.0 as f32, end.1 as f32),
                color,
            );
        }
    }

    fn center(&self) -> Point {
        if let Some(bounding) = self.bounding_box() {
            bounding.center();
        }

        Point(0.0, 0.0)
    }

    fn bounding_box(&self) -> Option<Rectangle> {
        if self.points.is_empty() {
            return None;
        }

        let p = self.points.get(0)?;

        let min_x = p.0;
        let min_y = p.1;
        let max_x = min_x;
        let max_y = min_y;

        let bounding =
            self.points
                .iter()
                .fold((min_x, min_y, max_x, max_y), |(x1, y1, x2, y2), point| {
                    (
                        x1.min(point.0),
                        y1.min(point.1),
                        x2.max(point.0),
                        y2.max(point.1),
                    )
                });

        Some(Rectangle::new(
            Point(bounding.0, bounding.1),
            bounding.2 - bounding.0,
            bounding.3 - bounding.1,
        ))
    }

    /**
    How this works: it starts by getting the bounding box for the polygon.
    After which it creates four search rays from the point out in each direction
    to the bounding box.

    It then takes two pairs of points (in other words a line) and checks how many
    times each search ray intersects with each line,
    if the intersection count is even, then the point is inside the polygon,
    if the intersection count is uneven, then the point is outside the polygon.
    **Note:** this is for each search ray, so if any ray has uneven hits
    the line is outside

    Illustrated below with an exaggerated bounding box for legabillity.
    ```
    /*
    -----------------------------
    |           |                |
    |       ____|______          |
    |      /    |      |         |
    |-----|-----*------/---------|
    |     |__   |     /          |
    |        |__|____/           |
    |           |                |
    -----------------------------
    */
    ```
    */
    fn contains(&self, point: &Point) -> bool {
        let bounds = if let Some(bounding) = self.bounding_box() {
            bounding
        } else {
            return false;
        };

        if !bounds.contains(point) {
            return false;
        }

        let search_rays = [
            (point, &Point(point.0, bounds.position.1)),
            (point, &Point(point.0, bounds.position.1 + bounds.height)),
            (point, &Point(bounds.position.0 + bounds.width, point.1)),
            (point, &Point(bounds.position.0, point.1)),
        ];

        for ray in search_rays {
            let mut intersections = 0;

            for i in 0..self.points.len() {
                match self.points.get(i + 1) {
                    None => break,
                    Some(_) => {
                        let line = (&self.points[i], &self.points[i + 1]);
                        if Path::intersects(line, ray) {
                            intersections += 1;
                        }
                    }
                }
            }

            if intersections % 2 == 0 {
                return false;
            }
        }
        true
    }
}
