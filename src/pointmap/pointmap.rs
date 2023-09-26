use std::fmt::Debug;

use crate::shapes::{point::Point, rectangle::Rectangle, shape::Shape};

#[derive(Debug)]
pub struct Pointmap<'points, 'bounds, T: Shape + Clone> {
    bounds: &'bounds Rectangle,
    points: &'points mut Vec<Vec<T>>,
    resolution: f64,
}

impl<'points, 'bounds, T: Shape + Debug + Clone> Pointmap<'points, 'bounds, T> {
    pub fn new(
        bounds: &'bounds Rectangle,
        points: &'points mut Vec<Vec<T>>,
        resolution: usize,
    ) -> Pointmap<'points, 'bounds, T> {
        Pointmap {
            bounds,
            points,
            resolution: resolution as f64,
        }
    }

    pub fn add_point(&mut self, shape: T) -> Result<(), String> {
        if !self.bounds.contains(&shape.center()) {
            return Err(format!(
                "Point {:?} is outside of the bounds {:?}",
                shape, self.bounds
            ));
        }

        let index = self.get_index(shape.center());

        if let Some(list) = self.points.get_mut(index) {
            list.push(shape);
        } else {
            self.points.push(vec![shape]);
        }

        Ok(())
    }

    pub fn get_neighbors(&self, shape: &T, max_distance: Option<f64>) -> Vec<&T> {
        let index = self.get_index(shape.center());

        self.get_neighboring_cells(index)
            .into_iter()
            .filter_map(|index| self.points.get(index))
            .flatten()
            .filter(|point| match max_distance {
                Some(max_distance) => point.center().distance_to(&shape.center()) < max_distance,
                None => true,
            })
            .collect::<Vec<&T>>()
    }

    pub fn points(&self) -> Vec<&T> {
        self.points.iter().flatten().collect::<Vec<&T>>()
    }

    fn get_neighboring_cells(&self, index: usize) -> Vec<usize> {
        let i = index as i32;
        let step = (self.points.len() as f64).sqrt() as i32;
        let over = i - step;
        let under = i + step;

        vec![
            over - 1,
            over,
            over + 1,
            i - 1,
            i,
            i + 1,
            under - 1,
            under,
            under + 1,
        ]
        .into_iter()
        .filter(|cell| *cell > 0 || (*cell as usize) < self.points.len())
        .map(|cell| cell as usize)
        .collect::<Vec<usize>>()
    }

    fn get_index(&self, point: Point) -> usize {
        let x =
            ((point.0 / (self.bounds.position.0 + self.bounds.width)) * self.resolution).floor();
        let y =
            ((point.1 / (self.bounds.position.1 + self.bounds.height)) * self.resolution).floor();

        (y * self.resolution + x - 1.0) as usize
    }
}
