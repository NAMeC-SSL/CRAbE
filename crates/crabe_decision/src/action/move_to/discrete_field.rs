use std::hash::{Hash, Hasher};
use std::io::Write;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use log::info;
use nalgebra::Point2;

pub struct DiscreteField<T> {
    pub data: Vec<Vec<T>>,
    resolution: f64,
    x_len: i32,
    y_len: i32,
    x_shift: f64,
    y_shift: f64
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x * y).round() / y
}


impl DiscreteField<CellData> {
    pub fn new(resolution: f64, field_length: f64, field_width: f64) -> Self {
        let l = field_length + 1.0;
        let w = field_width + 1.0;
        let x_len = (l / resolution) as i32;
        let y_len = (w / resolution) as i32;

        Self {
            data: vec![vec![CellData::default(); x_len as usize]; y_len as usize],
            resolution,
            x_len,
            y_len,
            x_shift: l / 2.0,
            y_shift: w / 2.0,
        }
    }

    pub fn idxs_to_coords(&self, i: i32, j: i32) -> Point2<f64> {
        Point2::new(round(j as f64 * self.resolution - self.x_shift, 3), round(i as f64 * self.resolution - self.y_shift, 3))
    }

    pub fn coords_to_idxs(&self, coords: &Point2<f64>) -> (usize, usize) {
        (round((coords.y + self.y_shift) / self.resolution, 3) as usize, round((coords.x + self.x_shift) / self.resolution, 3) as usize)
    }

    pub fn apply(&mut self, f: fn(&mut CellData)) {
        self.data.iter_mut().for_each(|row| row.iter_mut().for_each(f));
    }

    pub fn start(&mut self) -> Cursor {
        Cursor {
            field: Arc::new(self),
            pos: Default::default(),
            i: 0,
            j: 0,
        }
    }

    pub fn start_from(&mut self, x: f64, y: f64) -> Cursor {
        let i = ((y + self.y_shift) / self.resolution) as i32;
        let j = ((x + self.x_shift) / self.resolution) as i32;

        Cursor {
            field: Arc::new(self),
            pos: Default::default(),
            i,
            j
        }
    }

    // TODO: remove mut
    pub fn print(&self) {
        let mode = 1;
        let mut data = self.data.clone();

        let mut heatmap = std::fs::File::create("./test_astar.ppm").unwrap();
        heatmap.write_all(b"P3\n").unwrap();  // Portable pixel map mode
        heatmap.write_all(format!("{} {}\n", self.x_len, self.y_len - 1).as_bytes()).unwrap();  // Size of file
        heatmap.write_all(b"255\n").unwrap();  // Max number of color
        let mut range = 10.0;
        let mut alpha = 0.0;
        for j in (0..self.y_len as usize).rev()  {
            for i in 0..self.x_len as usize {
                // if data[j as usize][i as usize].weight.is_nan() {
                //     data[j as usize][i as usize].weight = 0.0;
                // }
                if mode == 1 {
                    if data[j][i].weight < 0.5 {
                        data[j][i].weight = 0.0;
                    } else if data[j][i].weight >= 0.5 && data[j][i].weight < 1.5 {
                        data[j][i].weight = 1.0;
                    } else if data[j][i].weight >= 1.5 && data[j][i].weight < 2.5 {
                        data[j][i].weight = 2.0;
                    } else if data[j][i].weight >= 2.5 && data[j][i].weight < 3.5 {
                        data[j][i].weight = 3.0;
                    } else if data[j][i].weight >= 3.5 && data[j][i].weight < 4.5 {
                        data[j][i].weight = 4.0;
                    } else if data[j][i].weight >= 4.5 && data[j][i].weight < 5.5 {
                        data[j][i].weight = 5.0;
                    } else if data[j][i].weight >= 5.5 && data[j][i].weight < 6.5 {
                        data[j][i].weight = 6.0;
                    } else if data[j][i].weight >= 6.5 && data[j][i].weight < 7.5 {
                        data[j][i].weight = 7.0;
                    } else if data[j][i].weight >= 7.5 && data[j][i].weight < 8.5 {
                        data[j][i].weight = 8.0;
                    } else if data[j][i].weight >= 8.5 && data[j][i].weight < 9.5 {
                        data[j][i].weight = 9.0;
                    } else if data[j][i].weight >= 9.5 {
                        data[j][i].weight = 10.0;
                    }

                    alpha = (range - data[j][i].weight).abs() / range;
                } else if mode == 0 {
                    if data[j][i].weight > 0.00001 && data[j][i].weight <= 10.0 {
                        alpha = (range - data[j][i].weight).abs() / range;
                    }
                } else {
                    println!("Incorrect mode for ppm");
                }

                let (mut R, mut G, mut B) = (0, 0, 0);
                if data[j][i].weight >= 9.5 {
                    R = 0;
                    G = 0;
                    B = 255;
                } else {
                    R = (alpha * 200. + (1. - alpha) * 255.0) as i32;
                    G = (alpha * 200. + (1. - alpha) * 0.0) as i32;
                    B = (alpha * 200. + (1. - alpha) * 0.0) as i32;
                }
                heatmap.write_all(format!("{:3} {:3} {:3} ", R, G, B).as_bytes()).unwrap();
            }
            heatmap.write_all(b"\n\n").unwrap();
        }
    }

    pub fn print_with_path(&self, path: &Vec<(usize, usize)>) {
        let mode = 1;
        let mut data = self.data.clone();

        let mut heatmap = std::fs::File::create("./test_astar.ppm").unwrap();
        heatmap.write_all(b"P3\n").unwrap();  // Portable pixel map mode
        heatmap.write_all(format!("{} {}\n", self.x_len, self.y_len - 1).as_bytes()).unwrap();  // Size of file
        heatmap.write_all(b"255\n").unwrap();  // Max number of color
        let mut range = 10.0;
        let mut alpha = 0.0;
        for j in (0..self.y_len as usize).rev()  {
            for i in 0..self.x_len as usize {
                // if data[j as usize][i as usize].weight.is_nan() {
                //     data[j as usize][i as usize].weight = 0.0;
                // }
                if mode == 1 {
                    if data[j][i].weight < 0.5 {
                        data[j][i].weight = 0.0;
                    } else if data[j][i].weight >= 0.5 && data[j][i].weight < 1.5 {
                        data[j][i].weight = 1.0;
                    } else if data[j][i].weight >= 1.5 && data[j][i].weight < 2.5 {
                        data[j][i].weight = 2.0;
                    } else if data[j][i].weight >= 2.5 && data[j][i].weight < 3.5 {
                        data[j][i].weight = 3.0;
                    } else if data[j][i].weight >= 3.5 && data[j][i].weight < 4.5 {
                        data[j][i].weight = 4.0;
                    } else if data[j][i].weight >= 4.5 && data[j][i].weight < 5.5 {
                        data[j][i].weight = 5.0;
                    } else if data[j][i].weight >= 5.5 && data[j][i].weight < 6.5 {
                        data[j][i].weight = 6.0;
                    } else if data[j][i].weight >= 6.5 && data[j][i].weight < 7.5 {
                        data[j][i].weight = 7.0;
                    } else if data[j][i].weight >= 7.5 && data[j][i].weight < 8.5 {
                        data[j][i].weight = 8.0;
                    } else if data[j][i].weight >= 8.5 && data[j][i].weight < 9.5 {
                        data[j][i].weight = 9.0;
                    } else if data[j][i].weight >= 9.5 {
                        data[j][i].weight = 10.0;
                    }

                    alpha = (range - data[j][i].weight).abs() / range;
                } else if mode == 0 {
                    if data[j][i].weight > 0.00001 && data[j][i].weight <= 10.0 {
                        alpha = (range - data[j][i].weight).abs() / range;
                    }
                } else {
                    println!("Incorrect mode for ppm");
                }

                let (mut R, mut G, mut B) = (0, 0, 0);
                if data[j][i].weight >= 9.5 {
                    R = 0;
                    G = 0;
                    B = 255;
                } else {
                    R = (alpha * 200. + (1. - alpha) * 255.0) as i32;
                    G = (alpha * 200. + (1. - alpha) * 0.0) as i32;
                    B = (alpha * 200. + (1. - alpha) * 0.0) as i32;
                }
                if path.contains(&(j, i)) {
                    G = 255;
                }
                heatmap.write_all(format!("{:3} {:3} {:3} ", R, G, B).as_bytes()).unwrap();
            }
            heatmap.write_all(b"\n\n").unwrap();
        }
    }
}

// impl Deref for DiscreteField<CellData> {
//     type Target = DiscreteField<CellData>;
//
//     fn deref(&self) -> &Self::Target {
//         self
//     }
// }
//
// impl DerefMut for DiscreteField<CellData> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self
//     }
// }

#[derive(Debug, Default, Clone)]
pub struct CellData {
    pub weight: f64,
    pub g_score: f64,
    pub visited: bool
}

#[derive(Clone)]
pub struct Cursor<'a> {
    field: Arc<&'a mut DiscreteField<CellData>>,
    pub pos: Point2<f64>,
    i: i32,
    j: i32
}

impl<'a> Cursor<'a> {
    fn new(i: i32, j: i32, field: Arc<&'a mut DiscreteField<CellData>>) -> Self {
        let mut c = Self {
            field,
            pos: Default::default(),
            i,
            j
        };

        c.update(i, j);
        c
    }

    fn update(&mut self, mut ni: i32, mut nj: i32) {
        if ni < 0 {
            ni = 0;
        }
        if ni >= self.field.y_len {
            ni = self.field.y_len - 1;
        }
        if nj < 0 {
            nj = 0;
        }
        if nj >= self.field.x_len {
            nj = self.field.x_len - 1;
        }

        self.i = ni;
        self.j = nj;

        self.pos = self.field.idxs_to_coords(self.i, self.j);
    }

    pub fn around(&self) -> Vec<Self> {
        let mut v = vec![];

        if self.i > 0 {
            v.push(Cursor::new( self.i - 1, self.j, self.field.clone()));
        }
        if self.i < self.field.y_len - 1 {
            v.push(Cursor::new(self.i + 1, self.j, self.field.clone()));
        }
        if self.j > 0 {
            v.push(Cursor::new(self.i, self.j - 1, self.field.clone()));
        }
        if self.j < self.field.x_len - 1 {
            v.push(Cursor::new(self.i, self.j + 1, self.field.clone()));
        }
        if self.i > 0 && self.j > 0 {
            v.push(Cursor::new(self.i - 1, self.j - 1, self.field.clone()));
        }
        if self.i > 0 && self.j < self.field.x_len - 1 {
            v.push(Cursor::new(self.i - 1, self.j + 1, self.field.clone()));
        }
        if self.i < self.field.y_len - 1 && self.j > 0 {
            v.push(Cursor::new(self.i + 1, self.j - 1, self.field.clone()));
        }
        if self.i < self.field.y_len - 1 && self.j < self.field.x_len - 1 {
            v.push(Cursor::new(self.i + 1, self.j + 1, self.field.clone()));
        }

        v
    }

    pub fn get(&self) -> &CellData {
        &self.field.data[self.i as usize][self.j as usize]
    }

    // pub fn get_mut(&mut self) -> &mut CellData {
    //     &mut self.field.data[self.i as usize][self.j as usize]
    // }
}

// impl<'a> Iterator for Cursor<'a> {
//     type Item = &'a mut CellData;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         self.j += 1;
//         if self.j == self.field.x_len {
//             self.i += 1;
//             self.j = 0;
//             if self.i == self.field.y_len {
//                 return None
//             }
//         }
//         self.update(self.i, self.j);
//         return Some(&mut self.field.data[self.i as usize][self.j as usize])
//     }
// }

impl PartialEq for Cursor<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}
impl Eq for Cursor<'_> {}

impl Hash for Cursor<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}

// impl Index<usize> for Cursor {
//     type Output = CellData;
//
//     fn index(&self, index: CellData) -> &Self::Output {
//         todo!()
//     }
//

#[cfg(test)]
mod test {
    use nalgebra::Point2;
    use crate::action::move_to::discrete_field::DiscreteField;

    #[test]
    fn check_conversion() {
        let field = DiscreteField::new(0.2, 9.0, 6.0);

        let p = (1, 2);
        let p_ = field.idxs_to_coords(p.0 as i32, p.1 as i32);
        let c = field.coords_to_idxs(&p_);

        assert!(dbg!(p) == dbg!(c));

        let p = (0, 0);
        let p_ = field.idxs_to_coords(p.0 as i32, p.1 as i32);
        let c = field.coords_to_idxs(&p_);

        assert!(dbg!(p) == dbg!(c));

        let p = (5, 7);
        let p_ = field.idxs_to_coords(p.0 as i32, p.1 as i32);
        let c = field.coords_to_idxs(&p_);

        assert!(dbg!(p) == dbg!(c));
    }

}
