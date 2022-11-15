use {
    bevy::math::Vec3A,
    rand::{rngs::StdRng, Rng},
    std::{
        error::Error,
        f32::consts::{PI, TAU},
        fmt::{Debug, Display},
    },
};

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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CartesianError::InvalidAlpha(alpha) => {
                f.write_fmt(format_args!("Alpha value must be [0..PI], was {}", alpha))
            },
        }
    }
}

pub fn cartesian_coordinates(
    alpha: f32,
    mut beta: f32,
    radius: f32,
) -> Result<Vec3A, CartesianError> {
    if alpha < 0.0 || alpha > PI {
        return Err(CartesianError::InvalidAlpha(alpha));
    }

    if beta < 0.0 {
        while beta < 0.0 {
            beta += PI;
        }
    } else {
        beta = beta.repeat(TAU);
    }

    let sin_alpha = f32::sin(alpha);

    Ok(Vec3A::new(
        sin_alpha * f32::cos(beta) * radius,
        f32::cos(alpha) * radius,
        sin_alpha * f32::sin(beta) * radius,
    ))
}

pub fn random_point_in_sphere(rng: &mut StdRng, radius: f32) -> Vec3A {
    // https://karthikkaranth.me/blog/generating-random-points-in-a-sphere/#better-choice-of-spherical-coordinates

    let u = rng.gen_range(0.0..1.0);
    let v = rng.gen_range(0.0..1.0);

    let theta = u * TAU;
    let phi = f32::acos(2.0 * v - 1.0);

    let r = f32::cbrt(rng.gen_range(0.0..radius));

    let sin_theta = f32::sin(theta);
    let cos_theta = f32::cos(theta);

    let sin_phi = f32::sin(phi);
    let cos_phi = f32::cos(phi);

    Vec3A::new(
        r * sin_phi * cos_theta,
        r * sin_phi * sin_theta,
        r * cos_phi,
    )
}

pub fn mix_values(a: f32, b: f32, weight_b: f32) -> f32 {
    (b * weight_b) + (a * (1.0 - weight_b))
}

pub trait RepeatNum {
    fn repeat(self, length: Self) -> Self;
}
impl RepeatNum for f32 {
    fn repeat(self, length: f32) -> f32 {
        f32::clamp(self - (self / length).floor() * length, 0.0, length)
    }
}
