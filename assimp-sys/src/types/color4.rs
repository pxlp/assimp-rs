use cgmath::Vector4;
use libc::c_float;

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AiColor4D {
    pub r: c_float,
    pub g: c_float,
    pub b: c_float,
    pub a: c_float
}

impl AiColor4D {
    pub fn to_cgmath_vector(&self) -> Vector4<c_float> {
        Vector4::new(self.r, self.g, self.b, self.a)
    }

    pub fn from_cgmath_vector(vector: &Vector4<c_float>) -> AiColor4D {
        AiColor4D {
            r: vector.x,
            g: vector.y,
            b: vector.z,
            a: vector.w
        }
    }
}
