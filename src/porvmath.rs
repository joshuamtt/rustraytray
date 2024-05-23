#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

pub mod porv_math {

    pub const POINT: f32 = 1.0;
    pub const VECTOR: f32 = 0.0;

    #[derive(Debug, PartialEq, Copy, Clone, Default)]
    pub struct PorvTuple {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32, // 1 = point, 0 = vector
    }
    impl PorvTuple {
        // I did not go with this design for now.
        // Not sure if it is better to have seperate functions for create a point, or vector
        // I think this is cleaner, since I don't have multiple types of the POINT or VECTOR
        // fn new(_x: f32, _y: f32, _z: f32, _w: f32) -> PorvTuple {
        //     match _w {
        //         POINT => PorvTuple {
        //             x: _x,
        //             y: _y,
        //             z: _z,
        //             w: _w,
        //         },
        //         VECTOR => PorvTuple {
        //             x: _x,
        //             y: _y,
        //             z: _z,
        //             w: _w,
        //         },
        //         _ => {
        //             panic!("Incorrect input, neither a POINT or VECTOR");
        //         }
        //     }
        // }
        //
        pub fn vector(x: f32, y: f32, z: f32) -> PorvTuple {
            PorvTuple { x, y, z, w: VECTOR }
        }

        pub fn point(x: f32, y: f32, z: f32) -> PorvTuple {
            PorvTuple { x, y, z, w: POINT }
        }

        pub fn magnitude(vector: &PorvTuple) -> f32 {
            if vector.w != VECTOR {
                panic!("Attempting to take the magnitude of something other than a vector!")
            }

            ((vector.x * vector.x) + (vector.y * vector.y) + (vector.z * vector.z)).sqrt()
        }

        pub fn normalize(vector: &mut PorvTuple) {
            let magnitude = PorvTuple::magnitude(vector);
            vector.x = vector.x / magnitude;
            vector.y = vector.y / magnitude;
            vector.z = vector.z / magnitude;
        }

        pub fn dot(a: PorvTuple, b: PorvTuple) -> f32 {
            if a.w != VECTOR && b.w != VECTOR {
                panic!("Attempting to take the dot product of something other than two vectors!")
            }

            a.x * b.x + a.y * b.y + a.z * b.z
        }

        pub fn cross(a: PorvTuple, b: PorvTuple) -> PorvTuple {
            PorvTuple {
                x: a.y * b.z - a.z * b.y,
                y: a.z * b.x - a.x * b.z,
                z: a.x * b.y - a.y * b.x,
                w: VECTOR,
            }
        }
    }
    impl std::ops::Add for PorvTuple {
        type Output = PorvTuple;

        fn add(self, other_tup: Self) -> Self {
            Self {
                x: self.x + other_tup.x,
                y: self.y + other_tup.y,
                z: self.z + other_tup.z,
                w: self.w + other_tup.w,
            }
        }
    }
    impl std::ops::Sub for PorvTuple {
        type Output = PorvTuple;

        // You can subtract a...
        // Point from a point
        // a vector from a point
        // and a vector from a vector
        fn sub(self, other_tup: Self) -> Self {
            if other_tup.w == POINT && self.w == VECTOR {
                panic!("Can't subtract a point from a vector!");
            }
            Self {
                x: self.x - other_tup.x,
                y: self.y - other_tup.y,
                z: self.z - other_tup.z,
                w: self.w - other_tup.w,
            }
        }
    }

    impl std::ops::Neg for PorvTuple {
        type Output = PorvTuple;

        fn neg(self) -> PorvTuple {
            PorvTuple {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: self.w, // no need to negate
            }
        }
    }
    impl std::ops::Mul<PorvTuple> for f32 {
        type Output = PorvTuple;
        // @ Now I am not sure how performant this is, I wonder if i can just take a pointer to the
        // rhs, and update the x y z values that way instead of creating a new PorvTuple?
        fn mul(self, rhs: PorvTuple) -> PorvTuple {
            PorvTuple {
                x: rhs.x * self,
                y: rhs.y * self,
                z: rhs.z * self,
                w: rhs.w,
            }
        }
    }

    // This will multiply two PorvTuples together but I am not sure I will need this.
    impl std::ops::Mul for PorvTuple {
        type Output = PorvTuple;

        fn mul(self, tuple: PorvTuple) -> PorvTuple {
            Self {
                x: self.x * tuple.x,
                y: self.y * tuple.y,
                z: self.z * tuple.z,
                w: self.w,
            }
        }
    }

    impl std::ops::Div<PorvTuple> for f32 {
        type Output = PorvTuple;

        fn div(self, rhs: PorvTuple) -> PorvTuple {
            PorvTuple {
                x: rhs.x / self,
                y: rhs.y / self,
                z: rhs.z / self,
                w: rhs.w,
            }
        }
    }

    impl std::fmt::Display for PorvTuple {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // For now this is how a PorvTuple will be printed...
            write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
        }
    }
}
