use std::fmt::{Display, Formatter};
use crate::utils::position::Position;

pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    elements: Vec<T>
}

impl<T: PartialEq + Copy> Grid<T> {
    pub fn new(height: usize, width: usize) -> Self {
        Grid { width, height, elements: Vec::with_capacity(width * height) }
    }

    pub fn fill(&mut self, element: T) {
        for _ in 0..self.height * self.width {
            self.elements.push(element);
        }
    }

    pub fn copy_and_fill(&mut self, element: T) -> Self {
        let mut grid = Grid::new(self.height, self.width);
        grid.fill(element);
        grid
    }

    pub fn insert(&mut self, point: Position, value: T) {
        let i = self.index(point);
        self.elements.insert(i, value);
    }

    pub fn set(&mut self, point: Position, value: T) {
        let i = self.index(point);
        self.elements[i] = value;
    }

    pub fn contains(&self, point: Position) -> bool {
        point.r >= 0 && point.c >= 0 && point.r < self.height as i32 && point.c < self.width as i32
    }

    pub fn get(&self, point: Position) -> Option<&T> {
        if !self.contains(point) { return None; }
        self.elements.get(self.index(point))
    }

    pub fn get_mut(&mut self, point: Position) -> Option<&mut T> {
        if !self.contains(point) { return None; }
        let i = self.index(point);
        self.elements.get_mut(i)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, &T)> {
        self.elements.iter().enumerate().map(|(i, e)| (self.point(i), e))
    }

    pub fn position(&self, element: &T) -> Option<Position> {
        self.elements.iter().position(|e| *e == *element).map(|idx| self.point(idx))
    }

    pub fn find(&self, element: &T) -> Option<(Position, &T)> {
        let maybe_idx = self.elements.iter().position(|e| *e == *element);
        maybe_idx.map(|idx| (self.point(idx), &self.elements[idx]))
    }

    fn index(&self, point: Position) -> usize {
        point.r as usize * self.width + point.c as usize
    }

    fn point(&self, index: usize) -> Position {
        Position::new((index / self.width) as i32, (index % self.width) as i32)
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let lines = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        let mut grid = Self::new(lines.len(), lines[0].len());
        for line in lines.iter() {
            grid.elements.extend_from_slice(line);
        }
        grid
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.elements.chunks(self.width) {
            writeln!(f, "{}", str::from_utf8(row).unwrap())?;
        }
        Ok(())
    }
}

impl<T: Clone> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Grid { width: self.width, height: self.height, elements: self.elements.clone() }
    }
}