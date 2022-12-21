use std::fmt::{self, Display, Write};

pub fn vec2<T: Default>(width: usize, height: usize) -> Vec<Vec<T>> {
    let mut vec = Vec::with_capacity(width);
    for i in 0..width {
        vec[i] = Vec::with_capacity(height);
    }
    return vec;
}

#[derive(Debug)]
pub struct Grid2<T> {
    vec2: Vec<Vec<T>>,
}

impl<T> Grid2<T> {
    pub fn from(vec2: Vec<Vec<T>>) -> Self {
        if vec2.len() == 0 {
            panic!("Grid height must be positive");
        }
        let len = vec2[0].len();
        if len == 0 {
            panic!("Grid width must be positive");
        }
        if !vec2.iter().all(|v| v.len() == len) {
            panic!("Invalid different vec lengths");
        }
        Grid2 { vec2 }
    }

    pub fn get(&self, w: usize, h: usize) -> Option<&T> {
        self.vec2.get(w)?.get(h)
    }

    pub fn get_mut(&mut self, w: usize, h: usize) -> Option<&mut T> {
        self.vec2.get_mut(w)?.get_mut(h)
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.vec2[0].len(), self.vec2.len())
    }
}

impl<T> Grid2<T>
where
    T: Default,
{
    pub fn with_dimensions(width: usize, height: usize) -> Grid2<T> {
        Grid2 {
            vec2: vec2::<T>(width, height),
        }
    }
}

impl<T: Display> Display for Grid2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut start = true;
        for v in self.vec2.iter() {
            if start {
                start = false;
            } else {
                f.write_char('\n')?
            }
            for t in v.iter() {
                t.fmt(f)?;
            }
        }
        Ok(())
    }
}
