use std::{
    error::Error,
    f32::consts::{PI, TAU},
    fmt::{Debug, Display},
};

use bevy::math::Vec3A;
use rand::Rng;

#[derive(Debug, Copy, Clone)]
pub enum CartesianError {
    InvalidAlpha(f32),
}
impl Error for CartesianError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
impl Display for CartesianError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CartesianError::InvalidAlpha(alpha) => {
                f.write_fmt(format_args!("Alpha value must be [0..PI], was {}", alpha))
            }
        }
    }
}

pub fn cartesian_coordinates(alpha: f32, beta: f32, radius: f32) -> Result<Vec3A, CartesianError> {
    if alpha < 0.0 || alpha > PI {
        return Err(CartesianError::InvalidAlpha(alpha));
    }
    let mut beta = beta.clone();

    while beta < 0.0 {
        beta += PI;
    }
    while beta >= TAU {
        beta -= TAU;
    }

    let sin_alpha = f32::sin(alpha);

    Ok(Vec3A::new(
        sin_alpha * f32::cos(beta) * radius,
        f32::cos(alpha) * radius,
        sin_alpha * f32::sin(beta) * radius,
    ))
}

pub fn random_point_in_sphere(radius: f32) -> Vec3A {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-radius..radius);
    let y = rng.gen_range(-radius..radius);
    let z = rng.gen_range(-radius..radius);
    let mult = 1.0 / (x * x + y * y + z * z).sqrt();

    if x == 0.0 && y == 0.0 && z == 0.0 {
        return Vec3A::X;
    }

    Vec3A::new(mult * x, mult * y, mult * z)
}
