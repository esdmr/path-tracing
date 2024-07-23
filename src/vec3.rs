use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::ppm::PPMColor;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn squared_abs(&self) -> f64 {
        self.dot(self)
    }

    pub fn abs(&self) -> f64 {
        self.squared_abs().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        self / self.abs()
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Vec3 index {index} out of bounds (> 2)"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Vec3 index {index} out of bounds (> 2)"),
        }
    }
}

impl Into<PPMColor> for Vec3 {
    fn into(self) -> PPMColor {
        PPMColor::new(
            (255. * self.0).trunc().clamp(0., 255.) as u8,
            (255. * self.1).trunc().clamp(0., 255.) as u8,
            (255. * self.2).trunc().clamp(0., 255.) as u8,
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

        impl $OpAssignTrait<f64> for Vec3 {
            fn $fn_op_assign(&mut self, rhs: f64) {
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

        impl $OpTrait<f64> for Vec3 {
            type Output = Self;

            fn $fn_op(mut self, rhs: f64) -> Self::Output {
                self.$fn_op_assign(rhs);
                self
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

        impl $OpTrait<f64> for &Vec3 {
            type Output = Vec3;

            fn $fn_op(self, rhs: f64) -> Self::Output {
                let mut result = self.clone();
                result.$fn_op_assign(rhs);
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
