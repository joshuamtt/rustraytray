#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use crate::matrices::matrix::*;

use crate::porvmath::porv_math::*;
mod matrices;
mod porvmath;

const EPSILON: f32 = 0.0001; // Used for comparing floats

use porvmath::porv_math;
use rand::{random, Rng};
use std::{io::Write, vec};

struct Projectile {
    position: PorvTuple, // POINT
    velocity: PorvTuple, // VECTOR
}
#[derive(Copy, Clone)]
struct Environment {
    gravity: PorvTuple, // VECTOR
    wind: PorvTuple,    // VECTOR
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Color>>, //
}

struct Ray {
    origin: PorvTuple,    // POINT!
    direction: PorvTuple, // VECTOR!
}

struct Intersection<T> {
    t_value: f32,
    object: T,
}

struct Sphere {
    // Sphere ID?????
    sphere_id: u32, // Random number.
    // position: PorvTuple, // POINT!
    transform: Matrix,
    material: Material,
}

struct PointLight {
    position: PorvTuple,
    intensity: Color,
}

struct Material {
    color: Color,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

fn main() {
    /*
    let env = Environment::new(
        PorvTuple::vector(0.0, -0.1, 0.0),
        PorvTuple::vector(-0.01, 0.0, 0.0),
    );

    let mut proj_velocity = PorvTuple::vector(1.0, 1.8, 0.0);
    PorvTuple::normalize(&mut proj_velocity);
    proj_velocity = 11.25 * proj_velocity; // @unchecked. can only multiply when the number is on the left hand side.

    let mut projc = Projectile::new(PorvTuple::point(0.0, 1.0, 0.0), proj_velocity);

    let color = Color::new(0.0, 0.0, 0.0);
    let mut canvas = Canvas::new(900, 550, color);
    let red = Color::new(1.0, 0.0, 0.0);

    while projc.position.y >= 0.0 {
        projc = tick(env, projc); // Update the projs position
        println!("({},{})", projc.position.x, projc.position.y);

        let y = canvas.height - projc.position.y as usize; // convert the y value coordinate system
        let write_pixel_result = canvas.write_pixel(projc.position.x as usize, y, red);
        match write_pixel_result {
            Ok(_) => {}
            Err(err) => {
                println!("Error: {err}");
                break;
            }
        }
    }
    let _ = Canvas::canvas_to_p3_ppm(canvas, String::from("output.ppm")).unwrap();
    */

    // This program will create a clock face with pixels where the hours are.

    // println!("{}", test == A);
}

fn tick(environment: Environment, projectile: Projectile) -> Projectile {
    Projectile {
        position: projectile.position + projectile.velocity,
        velocity: projectile.velocity + environment.gravity + environment.wind,
    }
}

fn float_equal(a: f32, b: f32) -> bool {
    if (a - b).abs() < EPSILON {
        true
    } else {
        false
    }
}

impl Projectile {
    fn new(point: PorvTuple, vector: PorvTuple) -> Projectile {
        if point.w != POINT && vector.w != VECTOR {
            panic!("Incorrect parameters to creating a projectile")
        }
        Projectile {
            position: point,
            velocity: vector,
        }
    }
}

impl Environment {
    fn new(gravity_vector: PorvTuple, wind_vector: PorvTuple) -> Environment {
        if gravity_vector.w != VECTOR && wind_vector.w != VECTOR {
            panic!("Incorrect parameters to creating an environment.")
        }
        Environment {
            gravity: gravity_vector,
            wind: wind_vector,
        }
    }
}

impl Color {
    fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    fn hadamard_product(c1: Color, c2: Color) -> Color {
        Color {
            r: c1.r * c2.r,
            g: c1.g * c2.g,
            b: c1.b * c2.b,
        }
    }
    // Scales a color value to a pixel value.
    fn scale_color_value(x: f32) -> i32 {
        match x {
            x if x < 0.0 => 0,
            x if x > 1.0 => 255,
            _ => (x * 255.0) as i32,
        }
    }
    // Will convert a color to a pixel.
    fn color_to_pixel(self) -> (i32, i32, i32) {
        (
            Color::scale_color_value(self.r),
            Color::scale_color_value(self.g),
            Color::scale_color_value(self.b),
        )
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, c2: Color) -> Color {
        Color {
            r: self.r - c2.r,
            g: self.g - c2.g,
            b: self.b - c2.b,
        }
    }
}

impl Ray {
    pub fn new(origin: PorvTuple, direction: PorvTuple) -> Ray {
        if origin.w != POINT {
            panic!("Origin was supplied something other than a point")
        }
        if direction.w != VECTOR {
            panic!("Direction was supplied something other than a point")
        }
        Ray { origin, direction }
    }
    pub fn position(ray: Ray, t: f32) -> PorvTuple {
        return ray.origin + t * ray.direction;
    }
    // Returns the collection of "t" values where ray intersects sphere.
    pub fn intersect(sphere: Sphere, ray: Ray) -> Vec<f32> {
        // Should only intersect, 2 times or is tangential (intersects at one point), or doesn't at all
        //
        //  for 2 intersections, count will be 2, and points will be there respectively.
        //  for 1, count = 2, and the point will be the same.
        //  for 0, count = 0
        let mut t_values: Vec<f32> = vec![];
        let sphere_to_ray = ray.origin - PorvTuple::point(0.0, 0.0, 0.0);
        let a = PorvTuple::dot(ray.direction, ray.direction);
        let b = 2.0 * PorvTuple::dot(ray.direction, sphere_to_ray);
        let c = PorvTuple::dot(sphere_to_ray, sphere_to_ray) - 1.0;

        let discriminant = (b * b) - 4.0 * a * c;

        if discriminant < 0.0 {
            // The ray misses the sphere.
            return t_values;
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        t_values[0] = t1;
        t_values[1] = t2;

        t_values
    }

    pub fn transform(ray: Ray, translation_matrix: Matrix) -> Ray {
        Ray {
            // TODO: Do something about this .clone stuff: should be fine for now though.
            direction: translation_matrix.clone() * ray.direction,
            origin: translation_matrix.clone() * ray.origin,
        }
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, c2: Color) -> Color {
        Color {
            r: self.r + c2.r,
            g: self.g + c2.g,
            b: self.b + c2.b,
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, c2: Color) -> Color {
        Color {
            r: self.r * c2.r,
            g: self.g * c2.g,
            b: self.b * c2.b,
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, c2: Color) -> Color {
        Color {
            r: self * c2.r,
            g: self * c2.g,
            b: self * c2.b,
        }
    }
}

impl Canvas {
    /*
    (y)
    |------------------
    |------------------
    |------------------
    |------------------
    |------------------
    |------------------
    |------------------ (x)

    The canvas pixels are setup as such, the y is the specific row, and x is the specific column.
    */
    fn new(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![vec![color; width]; height], // Height is the rows, width is the columns.
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: Color) -> Result<(), &'static str> {
        if y >= self.height || x >= self.width {
            return Err("Invalid Bounds.");
        }
        self.pixels[y][x] = color;

        Ok(())
    }

    fn pixel_at(self, x: usize, y: usize) -> Color {
        self.pixels[y][x]
    }

    fn canvas_to_p3_ppm(canvas: Canvas, file_name: String) -> std::io::Result<()> {
        let mut ppm_file = std::fs::File::create(file_name)?;
        // @unchecked.
        // Does this need to be mut?
        let mut pixel_tuple;

        ppm_file.write(format!("P3\n{} {}\n255\n", canvas.width, canvas.height).as_bytes())?;

        // Look into iterating over a 2d vector and stuff. first convert colors into pixels.
        for color_vec in canvas.pixels.iter() {
            for color in color_vec.iter() {
                pixel_tuple = color.color_to_pixel();
                ppm_file.write(
                    // Create a string of each part of the pixel, since this is P3
                    format!("{} {} {} ", pixel_tuple.0, pixel_tuple.1, pixel_tuple.2).as_bytes(),
                )?;
            }
            ppm_file.write("\n".as_bytes())?;
        }

        Ok(())
    }
}

impl PointLight {
    fn point_light(position: PorvTuple, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

impl Material {
    pub fn new(
        color: Color,
        ambient: f32,
        diffuse: f32,
        specular: f32,
        shininess: f32,
    ) -> Material {
        Material {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }
}

impl Sphere {
    fn new(material: Material) -> Sphere {
        Sphere {
            sphere_id: random(),
            transform: Matrix::identity_matrix(),
            material,
        }
    }
    // Assumes that the point will always be on the surface of the sphere.
    fn normal_at(sphere: Sphere, world_point: &mut PorvTuple) {
        let object_point = Matrix::inverse(&sphere.transform) * *world_point;
        let object_normal = object_point - PorvTuple::point(0.0, 0.0, 0.0);
        let mut world_normal =
            Matrix::transpose(&Matrix::inverse(&sphere.transform)) * object_normal;
        world_normal.w = 0.0;
        // why do i have to pass it as &mut when I declared as mut?
        return PorvTuple::normalize(&mut world_normal);
    }

    fn set_transform(sphere: &mut Sphere, transform: Matrix) {
        sphere.transform = transform;
    }
}
