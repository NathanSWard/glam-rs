#[macro_use]
mod macros;

use glam::{
    DMat2, DMat3, DMat4, DQuat, DVec2, DVec3, DVec4, Mat2, Mat3, Mat3A, Mat4, Quat, Vec2, Vec3,
    Vec3A, Vec4,
};

#[cfg(feature = "transform-types")]
use glam::{Affine2, Affine3A, DAffine2, DAffine3, Isometry3A, Transform3A};

pub trait Deg {
    fn to_radians(self) -> Self;
}

impl Deg for f32 {
    fn to_radians(self) -> f32 {
        f32::to_radians(self)
    }
}

impl Deg for f64 {
    fn to_radians(self) -> f64 {
        f64::to_radians(self)
    }
}

/// Helper function for migrating away from `glam::angle::deg`.
#[allow(dead_code)]
#[inline]
pub fn deg<T: Deg>(angle: T) -> T {
    angle.to_radians()
}

/// Helper function for migrating away from `glam::angle::rad`.
#[allow(dead_code)]
#[inline]
pub fn rad<T>(angle: T) -> T {
    angle
}

/// Trait used by the `assert_approx_eq` macro for floating point comparisons.
pub trait FloatCompare<Rhs: ?Sized = Self> {
    /// Return true if the absolute difference between `self` and `other` is
    /// less then or equal to `max_abs_diff`.
    fn approx_eq(&self, other: &Rhs, max_abs_diff: f32) -> bool;
    /// Returns the absolute difference of `self` and `other` which is printed
    /// if `assert_approx_eq` fails.
    fn abs_diff(&self, other: &Rhs) -> Rhs;
}

impl FloatCompare for f32 {
    #[inline]
    fn approx_eq(&self, other: &f32, max_abs_diff: f32) -> bool {
        (self - other).abs() <= max_abs_diff
    }
    #[inline]
    fn abs_diff(&self, other: &f32) -> f32 {
        (self - other).abs()
    }
}

impl FloatCompare for f64 {
    #[inline]
    fn approx_eq(&self, other: &f64, max_abs_diff: f32) -> bool {
        (self - other).abs() <= max_abs_diff as f64
    }
    #[inline]
    fn abs_diff(&self, other: &f64) -> f64 {
        (self - other).abs()
    }
}

impl FloatCompare for Mat2 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
        )
    }
}

impl FloatCompare for DMat2 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
        )
    }
}

impl FloatCompare for Mat3 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
            (self.z_axis - other.z_axis).abs(),
        )
    }
}

impl FloatCompare for Mat3A {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
            (self.z_axis - other.z_axis).abs(),
        )
    }
}

impl FloatCompare for DMat3 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
            (self.z_axis - other.z_axis).abs(),
        )
    }
}

impl FloatCompare for DMat4 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
            (self.z_axis - other.z_axis).abs(),
            (self.w_axis - other.w_axis).abs(),
        )
    }
}

#[cfg(feature = "transform-types")]
impl FloatCompare for Affine2 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            matrix2: self.matrix2.abs_diff(&other.matrix2),
            translation: self.translation.abs_diff(&other.translation),
        }
    }
}

#[cfg(feature = "transform-types")]
impl FloatCompare for DAffine2 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            matrix2: self.matrix2.abs_diff(&other.matrix2),
            translation: self.translation.abs_diff(&other.translation),
        }
    }
}

#[cfg(feature = "transform-types")]
impl FloatCompare for Affine3A {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            matrix3: self.matrix3.abs_diff(&other.matrix3),
            translation: self.translation.abs_diff(&other.translation),
        }
    }
}

#[cfg(feature = "transform-types")]
impl FloatCompare for DAffine3 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            matrix3: self.matrix3.abs_diff(&other.matrix3),
            translation: self.translation.abs_diff(&other.translation),
        }
    }
}

impl FloatCompare for Mat4 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_cols(
            (self.x_axis - other.x_axis).abs(),
            (self.y_axis - other.y_axis).abs(),
            (self.z_axis - other.z_axis).abs(),
            (self.w_axis - other.w_axis).abs(),
        )
    }
}

impl FloatCompare for Quat {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        let a: Vec4 = (*self).into();
        let b: Vec4 = (*other).into();
        (a - b).abs().into()
    }
}

impl FloatCompare for Vec2 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

impl FloatCompare for Vec3 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

impl FloatCompare for Vec3A {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

impl FloatCompare for Vec4 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

impl FloatCompare for DQuat {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        let a: DVec4 = (*self).into();
        let b: DVec4 = (*other).into();
        (a - b).abs().into()
    }
}

impl FloatCompare for DVec2 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

impl FloatCompare for DVec3 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

impl FloatCompare for DVec4 {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff as f64)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (*self - *other).abs()
    }
}

#[cfg(feature = "transform-types")]
impl FloatCompare for Transform3A {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }

    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_scale_rotation_translation(
            self.scale.abs_diff(&other.scale).into(),
            self.rotation.abs_diff(&other.rotation),
            self.translation.abs_diff(&other.translation).into(),
        )
    }
}

#[cfg(feature = "transform-types")]
impl FloatCompare for Isometry3A {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }

    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from_rotation_translation(
            self.rotation.abs_diff(&other.rotation),
            self.translation.abs_diff(&other.translation).into(),
        )
    }
}
