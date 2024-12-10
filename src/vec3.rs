use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{float::{random, Fl}, interval::Interval, ppm::PPMColor};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3(Fl, Fl, Fl);

impl Vec3 {
    pub const fn new(x: Fl, y: Fl, z: Fl) -> Self {
        Self(x, y, z)
    }

    pub const fn x(&self) -> Fl {
        self.0
    }

    pub const fn y(&self) -> Fl {
        self.1
    }

    pub const fn z(&self) -> Fl {
        self.2
    }

    pub fn dot(&self, rhs: &Self) -> Fl {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn squared_abs(&self) -> Fl {
        self.dot(self)
    }

    pub fn abs(&self) -> Fl {
        self.squared_abs().sqrt()
    }

    pub fn normalize(&self) -> Self {
        self / self.abs()
    }

    pub fn random() -> Self {
        Vec3(random(), random(), random())
    }

    pub fn random_between(interval: Interval) -> Self {
        Vec3(interval.random(), interval.random(), interval.random())
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_between(Interval::new(-1., 1.));

            if p.squared_abs() < 1. {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3(
                Interval::new(-1., 1.).random(),
                Interval::new(-1., 1.).random(),
                0.,
            );

            if p.squared_abs() < 1. {
                return p;
            }
        }
    }

    pub fn random_normalized() -> Self {
        Self::random_in_unit_sphere().normalize()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let vec = Self::random_normalized();

        if vec.dot(normal) > 0. {
            vec
        } else {
            -vec
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - normal * (2. * self.dot(normal))
    }

    pub fn refract(&self, normal: &Self, eta_i_over_eta_t: Fl) -> Self {
        let cos_theta = (-self).dot(normal).min(1.);
        let r_out_perpendicular = (*self + normal * cos_theta) * eta_i_over_eta_t;
        let r_out_parallel = normal * -(1. - r_out_perpendicular.squared_abs()).abs().sqrt();
        r_out_perpendicular + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl From<Vec3> for PPMColor {
    fn from(val: Vec3) -> Self {
        let intensity = Interval::new(0., 0.999);
        PPMColor::new(
            (256. * intensity.clamp(val.0.max(0.).sqrt())).trunc() as u8,
            (256. * intensity.clamp(val.1.max(0.).sqrt())).trunc() as u8,
            (256. * intensity.clamp(val.2.max(0.).sqrt())).trunc() as u8,
        )
    }
}

macro_rules! impl_math_op {
    ($OpAssignTrait:ident, $fn_op_assign:ident, $OpTrait:ident, $fn_op:ident) => {
        impl $OpAssignTrait for Vec3 {
            fn $fn_op_assign(&mut self, rhs: Self) {
                self.0.$fn_op_assign(rhs.0);
                self.1.$fn_op_assign(rhs.1);
                self.2.$fn_op_assign(rhs.2);
            }
        }

        impl $OpAssignTrait<Fl> for Vec3 {
            fn $fn_op_assign(&mut self, rhs: Fl) {
                self.0.$fn_op_assign(rhs);
                self.1.$fn_op_assign(rhs);
                self.2.$fn_op_assign(rhs);
            }
        }

        impl $OpTrait for Vec3 {
            type Output = Self;

            fn $fn_op(mut self, rhs: Self) -> Self::Output {
                self.$fn_op_assign(rhs);
                self
            }
        }

        impl $OpTrait<Fl> for Vec3 {
            type Output = Self;

            fn $fn_op(mut self, rhs: Fl) -> Self::Output {
                self.$fn_op_assign(rhs);
                self
            }
        }

        impl $OpTrait<Vec3> for Fl {
            type Output = Vec3;

            fn $fn_op(self, mut rhs: Vec3) -> Self::Output {
                rhs.$fn_op_assign(self);
                rhs
            }
        }

        impl $OpTrait for &Vec3 {
            type Output = Vec3;

            fn $fn_op(self, rhs: Self) -> Self::Output {
                let mut result = self.clone();
                result.$fn_op_assign(*rhs);
                result
            }
        }

        impl $OpTrait<Fl> for &Vec3 {
            type Output = Vec3;

            fn $fn_op(self, rhs: Fl) -> Self::Output {
                let mut result = self.clone();
                result.$fn_op_assign(rhs);
                result
            }
        }

        impl $OpTrait<&Vec3> for Fl {
            type Output = Vec3;

            fn $fn_op(self, rhs: &Vec3) -> Self::Output {
                let mut result = rhs.clone();
                result.$fn_op_assign(self);
                result
            }
        }
    };
}

impl_math_op!(AddAssign, add_assign, Add, add);
impl_math_op!(SubAssign, sub_assign, Sub, sub);
impl_math_op!(MulAssign, mul_assign, Mul, mul);
impl_math_op!(DivAssign, div_assign, Div, div);

pub type Pos3 = Vec3;
pub type Color = Vec3;
