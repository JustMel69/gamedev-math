#[macro_export]
macro_rules! gen_vec2 {
    ($ident:ident, $typ:ty, $zero:literal, $one:literal) => {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub struct $ident(pub $typ, pub $typ);

        impl $ident {
            pub fn x(self) -> $typ {
                return self.0;
            }

            pub fn y(self) -> $typ {
                return self.1;
            }

            pub fn set_x(&mut self, x: $typ) {
                self.0 = x;
            }

            pub fn set_y(&mut self, y: $typ) {
                self.1 = y;
            }

            pub fn xvec(self) -> Self {
                return Self(self.0, $zero);
            }

            pub fn yvec(self) -> Self {
                return Self($zero, self.1);
            }
        }

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.0, self.1)
            }
        }
    }
}

#[macro_export]
macro_rules! unsigned_vec2_impl {
    ($ident:ident, $typ:ty, $zero:literal, $one:literal) => {
        impl $ident {
            pub const ZERO: Self = Self($zero, $zero);
            pub const UP: Self = Self::up($one);
            pub const RIGHT: Self = Self::right($one);
            pub const ONE: Self = Self::one($one);
        
            pub const fn up(fact: $typ) -> Self {
                return Self($zero, fact);
            }
        
            pub const fn right(fact: $typ) -> Self {
                return Self(fact, $zero);
            }
        
            pub const fn one(fact: $typ) -> Self {
                return Self(fact, fact);
            }
        }
    };
}

#[macro_export]
macro_rules! signed_vec2_impl {
    ($ident:ident, $typ:ty, $zero:literal, $one:literal) => {
        impl $ident {
            pub const DOWN: Self = Self::down($one);
            pub const LEFT: Self = Self::left($one);
            
            pub const fn down(fact: $typ) -> Self {
                return Self($zero, -fact);
            }
        
            pub const fn left(fact: $typ) -> Self {
                return Self(-fact, $zero);
            }
        }
    };
}

#[macro_export]
macro_rules! scalar_vec2_impl {
    ($ident:ident, $typ:ty, $styp:ty) => {
        impl $ident {
            /// Creates a vector in the local space of a basis
            pub const fn local(x: $typ, y: $typ, basis: (Self, Self)) -> Self {
                return Self(x * basis.0.0 + y * basis.1.0, x * basis.0.1 + y * basis.1.1)
            }

            /// Performs the dot product between two vectors.
            pub fn dot(self, other: Self) -> $typ {
                return (self.to_simd() * other.to_simd()).reduce_sum();
            }

            pub fn sqr_magnitude(self) -> $typ {
                return self.dot(self);
            }


            /// Multiplies the vectors component-wise
            pub fn scale(self, other: Self) -> Self {
                return Self::from_simd(self.to_simd() * other.to_simd());
            }

            /// Divides the vectors component-wise
            pub fn inv_scale(self, other: Self) -> Self {
                return Self::from_simd(self.to_simd() / other.to_simd());
            }


            pub fn min(self, other: Self) -> Self {
                Self::from_simd(self.to_simd().simd_min(other.to_simd()))
            }
        
            pub fn max(self, other: Self) -> Self {
                Self::from_simd(self.to_simd().simd_max(other.to_simd()))
            }
        
            pub fn clamp(self, min: Self, max: Self) -> Self {
                self.max(min).min(max)
            }

            
            pub fn max_axis(self) -> $typ {
                return self.to_simd().reduce_max();
            }
        
            pub fn min_axis(self) -> $typ {
                return self.to_simd().reduce_min();
            }


            pub fn to_simd(self) -> $styp {
                <$styp>::from_array([self.0, self.1])
            }
        
            pub fn from_simd(simd: $styp) -> Self {
                Self(simd[0], simd[1])
            }
        }

        impl std::ops::Add for $ident {
            type Output = Self;
        
            fn add(self, rhs: Self) -> Self::Output {
                return Self::from_simd(self.to_simd() + rhs.to_simd());
            }
        }
        
        impl std::ops::AddAssign for $ident {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs
            }
        }
        
        impl std::ops::Sub for $ident {
            type Output = Self;
        
            fn sub(self, rhs: Self) -> Self::Output {
                return Self::from_simd(self.to_simd() - rhs.to_simd());
            }
        }

        impl std::ops::SubAssign for $ident {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs
            }
        }

        impl std::ops::Mul<$typ> for $ident {
            type Output = Self;
        
            fn mul(self, rhs: $typ) -> Self::Output {
                return Self::from_simd(self.to_simd() * <$styp>::splat(rhs));
            }
        }

        impl std::ops::MulAssign<$typ> for $ident {
            fn mul_assign(&mut self, rhs: $typ) {
                *self = *self * rhs
            }
        }
        
        impl std::ops::Div<$typ> for $ident {
            type Output = Self;
        
            fn div(self, rhs: $typ) -> Self::Output {
                return Self::from_simd(self.to_simd() / <$styp>::splat(rhs));
            }
        }

        impl std::ops::DivAssign<$typ> for $ident {
            fn div_assign(&mut self, rhs: $typ) {
                *self = *self * rhs
            }
        }
        
        impl std::ops::From<($typ, $typ)> for $ident {
            fn from(value: ($typ, $typ)) -> Self {
                Self(value.0, value.1)
            }
        }
        
        impl std::ops::From<$ident> for ($typ, $typ) {
            fn from(value: $ident) -> Self {
                (value.0, value.1)
            }
        }
    };
}

#[macro_export]
macro_rules! float_vec2_impl {
    ($ident:ident, $typ:ty, $styp:ty) => {
        impl $ident {
            pub fn cross(self) -> Self {
                return Self(self.1, -self.0);
            }
        
            /// Rotates the vector by the rotation provided in radians.
            pub fn rotate(self, rot: $typ) -> Self {
                return Self(self.0 * rot.cos() + self.1 * rot.sin(), self.0 * rot.sin() - self.1 * rot.cos());
            }
        
            /// Rotates the vector by the specified rotation, but in a way that matches up with visual rotations.
            pub fn rotate_cw(self, rot: $typ) -> Self {
                let rotated = self.rotate(rot);
                return Self(rotated.0, -rotated.1);
            }
        
        
            /// Returns the magnitude of the vector.
            pub fn magnitude(self) -> $typ {
                if self == Self::ZERO {
                    return 0.0;
                }
        
                return (self.to_simd() * self.to_simd()).reduce_sum().sqrt();
            }
        
            /// Returns the vector with a magnitude of 1.
            pub fn normalized(self) -> Self {
                let mag = self.magnitude();
                if mag == 0.0 {
                    return self;
                }
        
                return self / mag;
            }
        
            /// Returns the vector divided by the squared magnitude.
            pub fn inverted(self) -> Self {
                let sqr_mag = self.dot(self);
                if sqr_mag == 0.0 {
                    return self;
                }
        
                return self / sqr_mag;
            }
        
            /// Inverts each component
            pub fn inv_dims(self) -> Self {
                return Self::from_simd(<$styp>::splat(1.0) / self.to_simd());
            }
        
            pub fn average(slice: &[Self]) -> Self {
                let mut res = Self::ZERO;
                for v in slice {
                    res += *v;
                }
                return res / slice.len() as $typ;
            }
        
            pub fn dist_to(self, other: Self) -> $typ {
                (self - other).magnitude()
            }
        
            pub fn sqr_dist_to(self, other: Self) -> $typ {
                (self - other).sqr_magnitude()
            }
        
        
            
            pub fn min_mag(self, other: $typ) -> Self {
                let mag = self.magnitude();
                return self / mag * mag.min(other);
            }
        
            pub fn max_mag(self, other: $typ) -> Self {
                let mag = self.magnitude();
                return self / mag * mag.max(other);
            }
        
            pub fn clamp_mag(self, min: $typ, max: $typ) -> Self {
                let mag = self.magnitude();
                return self / mag * mag.clamp(min, max);
            }
        
        
            pub fn floor(self) -> Self {
                return Self::from_simd(self.to_simd().floor());
            }
        
            pub fn round(self) -> Self {
                return Self::from_simd(self.to_simd().round());
            }
        
            pub fn ceil(self) -> Self {
                return Self::from_simd(self.to_simd().ceil());
            }
        }

        impl Neg for $ident {
            type Output = Self;
        
            fn neg(self) -> Self::Output {
                return Self::from_simd(-self.to_simd());
            }
        }
    };
}

#[macro_export]
macro_rules! cast_vec2_impl {
    ($ident:ident, $typ:ty, $vec_a:ty, $vec_b:ty, $vec_c:ty) => {
        impl From<$vec_a> for $ident {
            fn from(value: $vec_a) -> Self {
                Self(value.0 as $typ, value.1 as $typ)
            }
        }

        impl From<$vec_b> for $ident {
            fn from(value: $vec_b) -> Self {
                Self(value.0 as $typ, value.1 as $typ)
            }
        }

        impl From<$vec_c> for $ident {
            fn from(value: $vec_c) -> Self {
                Self(value.0 as $typ, value.1 as $typ)
            }
        }
    };
}