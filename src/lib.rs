//! # Regular- & Semi-Regular Tilings
//! This crate creates 2D meshes of all of the regular and semi-regular
//! tilings of the plane.
//!
//! A
//! [regular tiling](https://en.wikipedia.org/wiki/Euclidean_tilings_by_convex_regular_polygons#Regular_tilings)
//! is one consisting of only a single regular polygon with unit length
//! edges.
//!
//! A
//! [semi-regular tiling](https://en.wikipedia.org/wiki/Euclidean_tilings_by_convex_regular_polygons#Archimedean,_uniform_or_semiregular_tilings)
//! may contain more than one type of polygon, but each vertex will
//! look identical (up to rotation).
//!
use core::f64::consts::SQRT_2;
#[cfg(feature = "obj")]
use std::{error::Error, io::Write};
use ultraviolet as uv;

type VertexKey = u32;
pub type Face = Vec<VertexKey>;
pub type FaceIndex = Vec<Face>;
pub type Point = uv::Vec2;
pub type Points = Vec<Point>;

macro_rules! default_methods {
    () => {
        pub fn faces(&self) -> &FaceIndex {
            &self.face_index
        }

        pub fn points(&self) -> &Points {
            &self.points
        }

        pub fn name(&self) -> &str {
            self.name.as_str()
        }

        #[cfg(feature = "obj")]
        pub fn to_obj(&self, reverse_face_winding: bool) -> Result<Vec<u8>, Box<dyn Error>> {
            let mut file = Vec::new();

            writeln!(file, "o {}-tiling", self.name)?;

            for vertex in &self.points {
                writeln!(file, "v {} {} 0", vertex.x, vertex.y)?;
            }

            if reverse_face_winding {
                for face in &self.face_index {
                    write!(file, "f")?;
                    for vertex_index in face.iter().rev() {
                        write!(file, " {}", vertex_index + 1)?;
                    }
                    writeln!(file)?;
                }
            } else {
                for face in &self.face_index {
                    write!(file, "f")?;
                    for vertex_index in face {
                        write!(file, " {}", vertex_index + 1)?;
                    }
                    writeln!(file)?;
                }
            }

            Ok(file)
        }
    };
}

const SQRT_3: f64 = 1.732_050_807_568_877_293;

pub struct SemiRegularTiling {
    face_index: FaceIndex,
    points: Points,
    name: String,
}

impl SemiRegularTiling {
    default_methods! {}

    /// Creates the 1st semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/7/70/1-uniform_n10.svg/1920px-1-uniform_n10.svg.png)
    pub fn one(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-1".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            (x as f64 + 0.5 * y as f64) as f32,
                            ((y as f64 * SQRT_3) * 0.5) as _,
                        )
                    })
                })
                .collect(),

            face_index: (1..rows - 1)
                .flat_map(|y| {
                    (1..cols - 1)
                        .filter_map(move |x| {
                            let i = (x + 3 * y) % 7;
                            let mut result = Vec::new();
                            if i == 0 || i == 2 || i >= 5 {
                                result.push(vec![
                                    ((x + 0) + (y + 0) * cols),
                                    ((x + 1) + (y + 0) * cols),
                                    ((x + 0) + (y + 1) * cols),
                                ]);
                            }
                            if i == 2 || i >= 4 {
                                result.push(vec![
                                    ((x + 1) + (y + 0) * cols),
                                    ((x + 1) + (y + 1) * cols),
                                    ((x + 0) + (y + 1) * cols),
                                ]);
                            }
                            if i == 4 {
                                result.push(vec![
                                    ((x + 1) + (y + 0) * cols),
                                    ((x + 0) + (y + 1) * cols),
                                    ((x - 1) + (y + 1) * cols),
                                    ((x - 1) + (y + 0) * cols),
                                    ((x + 0) + (y - 1) * cols),
                                    ((x + 1) + (y - 1) * cols),
                                ]);
                            }
                            if result.is_empty() {
                                None
                            } else {
                                Some(result)
                            }
                        })
                        .flatten()
                })
                .collect(),
        }
    }

    /// Creates the 2nd semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/9/95/1-uniform_n2.svg/1920px-1-uniform_n2.svg.png)
    pub fn two(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-2".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            ((x >> 1) as f64 * (2.0 + SQRT_2)
                                + (x % 2) as f64
                                + (y >> 1) as f64 * (1.0 + SQRT_2 * 0.5))
                                as f32,
                            ((y >> 1) as f64 * (1.0 + SQRT_2 * 0.5) + (y % 2) as f64) as f32,
                        )
                    })
                })
                .collect(),

            face_index: (1..rows - 2)
                .flat_map(|y| {
                    (1..cols - 2).filter_map(move |x| {
                        if x % 2 == 1 && y % 2 == 0 {
                            Some(vec![
                                (x + 0) + (y + 0) * cols,
                                (x + 1) + (y - 1) * cols,
                                (x + 2) + (y - 1) * cols,
                                (x + 1) + (y + 0) * cols,
                                (x + 1) + (y + 1) * cols,
                                (x + 0) + (y + 2) * cols,
                                (x - 1) + (y + 2) * cols,
                                (x + 0) + (y + 1) * cols,
                            ])
                        } else {
                            None
                        }
                    })
                })
                .chain((0..rows - 1).flat_map(|y| {
                    (0..cols - 1).filter_map(move |x| {
                        if x % 2 == 0 && y % 2 == 0 {
                            Some(vec![
                                (x + 0) + (y + 0) * cols,
                                (x + 1) + (y + 0) * cols,
                                (x + 1) + (y + 1) * cols,
                                (x + 0) + (y + 1) * cols,
                            ])
                        } else {
                            None
                        }
                    })
                }))
                .collect(),
        }
    }

    /// Creates the 3rd semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/c/c6/1-uniform_n8.svg/1920px-1-uniform_n8.svg.png)
    pub fn three(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-3".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            (x as f64 + 0.5 * (y >> 1) as f64) as f32,
                            ((y >> 1) as f64 * (1.0 + SQRT_3 * 0.5) + (y % 2) as f64) as _,
                        )
                    })
                })
                .collect(),

            face_index: (0..rows - 1)
                .flat_map(|y| {
                    (0..cols - 1).flat_map(move |x| {
                        if y % 2 == 0 {
                            vec![vec![
                                (x + 0) + (y + 0) * cols,
                                (x + 1) + (y + 0) * cols,
                                (x + 1) + (y + 1) * cols,
                                (x + 0) + (y + 1) * cols,
                            ]]
                        } else {
                            vec![
                                vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ],
                                vec![
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ],
                            ]
                        }
                    })
                })
                .collect(),
        }
    }

    /// Creates the 4th semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/3/34/1-uniform_n7.svg/1920px-1-uniform_n7.svg.png)
    pub fn four(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-4".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            (x as f64 + 0.5 * y as f64) as f32,
                            ((y as f64 * SQRT_3) * 0.5) as _,
                        )
                    })
                })
                .collect(),

            face_index: (1..rows - 2)
                .flat_map(|y| {
                    (1..cols - 1)
                        .filter_map(move |x| {
                            let mut result = Vec::new();

                            if x % 2 == 0 && y % 2 == 0 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 0 && y % 2 == 1 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 0) + (y + 1) * cols,
                                    (x - 1) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 1 && y % 2 == 0 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 2) * cols,
                                    (x - 1) + (y + 2) * cols,
                                    (x - 1) + (y + 1) * cols,
                                ]);
                            }

                            if result.is_empty() {
                                None
                            } else {
                                Some(result)
                            }
                        })
                        .flatten()
                })
                .collect(),
        }
    }

    /// Creates the 5th semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/6/6c/1-uniform_n9.svg/1920px-1-uniform_n9.svg.png)
    pub fn five(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-5".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            ((x >> 1) as f64 * (1.0 + SQRT_3 * 0.5) + (x % 2) as f64
                                - (y >> 1) as f64 * 0.5) as f32,
                            ((y >> 1) as f64 * (1.0 + SQRT_3 * 0.5)
                                + (y % 2) as f64
                                + (x >> 1) as f64 * 0.5) as _,
                        )
                    })
                })
                .collect(),

            face_index: (0..rows - 1)
                .flat_map(|y| {
                    (1..cols - 1)
                        .filter_map(move |x| {
                            let mut result = Vec::new();

                            if x % 2 == 0 && y % 2 == 0 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 0) + (y + 1) * cols,
                                    (x - 1) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 1 && y % 2 == 0 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 0 && y % 2 == 1 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                ]);
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 1 && y % 2 == 1 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }

                            if result.is_empty() {
                                None
                            } else {
                                Some(result)
                            }
                        })
                        .flatten()
                })
                .collect(),
        }
    }

    /// Creates the 6th semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/2/24/1-uniform_n4.svg/1920px-1-uniform_n4.svg.png)
    pub fn six(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-6".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        let mut px = (x >> 1) as f64 * (2.0 + SQRT_3)
                            + (x % 2) as f64
                            + (y >> 2) as f64 * (1.0 + SQRT_3 * 0.5);
                        let mut py = (y >> 2) as f64 * (1.5 + SQRT_3);

                        match y % 4 {
                            0 => (),
                            1 => {
                                px += 0.5;
                                py += SQRT_3 * 0.5;
                            }
                            2 => {
                                px += 0.5;
                                py += 1.0 + SQRT_3 * 0.5;
                            }
                            3 => py += 1.0 + SQRT_3,
                            _ => unreachable!(),
                        }

                        Point::new(px as f32, py as _)
                    })
                })
                .collect(),

            face_index: (1..rows - 4)
                .flat_map(|y| {
                    (0..cols - 3)
                        .filter_map(move |x| {
                            let mut result = Vec::new();

                            if x % 2 == 0 && y % 4 == 0 {
                                result.push(vec![
                                    (x + 1) + (y + 0) * cols,
                                    (x + 2) + (y - 1) * cols,
                                    (x + 3) + (y - 1) * cols,
                                    (x + 2) + (y + 0) * cols,
                                    (x + 2) + (y + 1) * cols,
                                    (x + 2) + (y + 2) * cols,
                                    (x + 2) + (y + 3) * cols,
                                    (x + 1) + (y + 4) * cols,
                                    (x + 0) + (y + 4) * cols,
                                    (x + 1) + (y + 3) * cols,
                                    (x + 0) + (y + 2) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);

                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 0 && y % 4 == 2 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }

                            if result.is_empty() {
                                None
                            } else {
                                Some(result)
                            }
                        })
                        .flatten()
                })
                .collect(),
        }
    }

    /// Creates the 7th semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/d/d9/1-uniform_n6.svg/1920px-1-uniform_n6.svg.png)
    pub fn seven(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-7".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        let mut px = (x >> 1) as f64 * (1.0 + SQRT_3)
                            + (x % 2) as f64 * SQRT_3
                            + (y >> 2) as f64 * (0.5 + SQRT_3 * 0.5);
                        let mut py = (y >> 2) as f64 * (1.5 + SQRT_3 * 0.5);

                        match y % 4 {
                            0 => {
                                px += SQRT_3 * 0.5;
                                py -= 0.5;
                            }
                            1 => (),
                            2 => py += 1.0,
                            3 => {
                                px += SQRT_3 * 0.5;
                                py += 1.5;
                            }
                            _ => unreachable!(),
                        }

                        Point::new(px as f32, py as _)
                    })
                })
                .collect(),

            face_index: (2..rows - 3)
                .flat_map(|y| {
                    (2..cols - 2)
                        .filter_map(move |x| {
                            let mut result = Vec::new();

                            if x % 2 == 0 && y % 4 == 0 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 1) + (y + 2) * cols,
                                    (x + 0) + (y + 3) * cols,
                                    (x + 0) + (y + 2) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 2) + (y - 2) * cols,
                                    (x + 2) + (y - 1) * cols,
                                    (x + 1) + (y + 1) * cols,
                                ]);
                                result.push(vec![
                                    (x + 1) + (y + 1) * cols,
                                    (x + 2) + (y - 1) * cols,
                                    (x + 2) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 1 && y % 4 == 1 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 1 && y % 4 == 2 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x - 1) + (y + 2) * cols,
                                    (x - 1) + (y + 3) * cols,
                                    (x - 1) + (y + 1) * cols,
                                ]);
                            }
                            if x % 2 == 0 && y % 4 == 2 {
                                result.push(vec![
                                    (x - 1) + (y + 0) * cols,
                                    (x + 0) + (y + 0) * cols,
                                    (x - 2) + (y + 2) * cols,
                                ]);
                            }

                            if result.is_empty() {
                                None
                            } else {
                                Some(result)
                            }
                        })
                        .flatten()
                })
                .collect(),
        }
    }

    /// Creates the 8th semi-regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/1-uniform_n3.svg/1920px-1-uniform_n3.svg.png)
    pub fn eight(rows: u32, cols: u32) -> Self {
        Self {
            name: "SEMI-REGULAR-8".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        let mut px = (x >> 2) as f64 * (3.0 + 3.0 * SQRT_3)
                            + (y >> 2) as f64 * (1.5 + 1.5 * SQRT_3);

                        let mut py = (y >> 2) as f64 * (1.5 + SQRT_3 * 0.5);

                        match y % 4 {
                            0 => {
                                px += SQRT_3 * 0.5;
                                py -= 0.5;
                            }
                            1 => (),
                            2 => py += 1.0,
                            3 => {
                                px += SQRT_3 * 0.5;
                                py += 1.5;
                            }
                            _ => unreachable!(),
                        }

                        match x % 4 {
                            0 => (),
                            1 => px += SQRT_3,
                            2 => px += 1.0 + SQRT_3,
                            3 => px += 1.0 + 2.0 * SQRT_3,
                            _ => unreachable!(),
                        }

                        Point::new(px as f32, py as _)
                    })
                })
                .collect(),

            face_index: (3..rows - 4)
                .flat_map(|y| {
                    (3..cols - 3)
                        .filter_map(move |x| {
                            let mut result = Vec::new();

                            if x % 2 == 0 && y % 4 == 0 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 1) + (y + 2) * cols,
                                    (x + 0) + (y + 3) * cols,
                                    (x + 0) + (y + 2) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 4 == 1 && y % 4 == 1 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }
                            if x % 4 == 3 && y % 4 == 2 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x - 3) + (y + 2) * cols,
                                    (x - 3) + (y + 3) * cols,
                                    (x - 1) + (y + 1) * cols,
                                ]);
                            }
                            if x % 4 == 0 && y % 4 == 2 {
                                result.push(vec![
                                    (x + 0) + (y + 1) * cols,
                                    (x - 1) + (y + 3) * cols,
                                    (x - 2) + (y + 2) * cols,
                                    (x + 0) + (y + 0) * cols,
                                ]);
                            }
                            if x % 4 == 3 && y % 4 == 1 {
                                result.push(vec![
                                    (x + 0) + (y + 0) * cols,
                                    (x + 1) + (y - 2) * cols,
                                    (x + 2) + (y - 3) * cols,
                                    (x + 3) + (y - 3) * cols,
                                    (x + 3) + (y - 2) * cols,
                                    (x + 1) + (y + 0) * cols,
                                    (x + 1) + (y + 1) * cols,
                                    (x - 1) + (y + 3) * cols,
                                    (x - 1) + (y + 4) * cols,
                                    (x - 2) + (y + 4) * cols,
                                    (x - 3) + (y + 3) * cols,
                                    (x + 0) + (y + 1) * cols,
                                ]);
                            }

                            if result.is_empty() {
                                None
                            } else {
                                Some(result)
                            }
                        })
                        .flatten()
                })
                .collect(),
        }
    }
}

pub struct RegularTiling {
    face_index: FaceIndex,
    points: Points,
    name: String,
}

impl RegularTiling {
    default_methods! {}

    /// Creates the triangle regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/ac/1-uniform_n11.svg/1920px-1-uniform_n11.svg.png)
    pub fn triangle(rows: u32, cols: u32) -> Self {
        Self {
            name: "TRIANGLE".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            (x as f64 + 0.5 * y as f64) as _,
                            (y as f64 * SQRT_3 * 0.5) as _,
                        )
                    })
                })
                .collect(),

            face_index: (0..rows - 1)
                .flat_map(|y| {
                    (0..cols - 1).flat_map(move |x| {
                        vec![
                            vec![
                                (x + 0) + (y + 0) * cols,
                                (x + 1) + (y + 0) * cols,
                                (x + 0) + (y + 1) * cols,
                            ],
                            vec![
                                (x + 1) + (y + 0) * cols,
                                (x + 1) + (y + 1) * cols,
                                (x + 0) + (y + 1) * cols,
                            ],
                        ]
                    })
                })
                .collect(),
        }
    }

    /// Creates the square regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/c/c4/1-uniform_n5.svg/1920px-1-uniform_n5.svg.png)
    pub fn square(rows: u32, cols: u32) -> Self {
        Self {
            name: "SQUARE".to_string(),
            points: (0..rows)
                .flat_map(|y| (0..cols).map(move |x| Point::new(x as _, y as _)))
                .collect(),

            face_index: (0..rows - 1)
                .flat_map(|y| {
                    (0..cols - 1).map(move |x| {
                        vec![
                            (x + 0) + (y + 0) * cols,
                            (x + 1) + (y + 0) * cols,
                            (x + 1) + (y + 1) * cols,
                            (x + 0) + (y + 1) * cols,
                        ]
                    })
                })
                .collect(),
        }
    }

    /// Creates the hexagon regular tiling.
    ///
    /// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a6/1-uniform_n1.svg/1920px-1-uniform_n1.svg.png)
    pub fn hexagon(rows: u32, cols: u32) -> Self {
        Self {
            name: "HEXAGON".to_string(),
            points: (0..rows)
                .flat_map(|y| {
                    (0..cols).map(move |x| {
                        Point::new(
                            (((x + (y % 2)) as f64 / 2.0).floor() * 3.0 + ((x + y) % 2) as f64
                                - (y % 2) as f64 * 1.5) as _,
                            (y as f64 * SQRT_3 * 0.5) as _,
                        )
                    })
                })
                .collect(),
            face_index: (0..rows - 2)
                .flat_map(|y| {
                    (0..cols - 1).filter_map(move |x| {
                        if (y % 2 == 0 && x % 2 == 0) || (y % 2 == 1 && x % 2 == 1) {
                            Some(vec![
                                (x + 0) + (y + 0) * cols,
                                (x + 1) + (y + 0) * cols,
                                (x + 1) + (y + 1) * cols,
                                (x + 1) + (y + 2) * cols,
                                (x + 0) + (y + 2) * cols,
                                (x + 0) + (y + 1) * cols,
                            ])
                        } else {
                            None
                        }
                    })
                })
                .collect(),
        }
    }
}

#[test]
#[cfg(feature = "obj")]
pub fn obj() -> Result<(), Box<dyn Error>> {
    use std::fs::File;

    let tiling = RegularTiling::triangle(100, 100);
    let mut file = File::create(format!("./{}.obj", tiling.name()))?;
    file.write_all(&tiling.to_obj(false)?)?;
    file.flush()?;

    let tiling = SemiRegularTiling::seven(100, 100);
    let mut file = File::create(format!("./{}.obj", tiling.name()))?;
    file.write_all(&tiling.to_obj(false)?)?;
    file.flush()?;

    Ok(())
}
