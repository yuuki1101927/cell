use crate::Vector3d;

pub struct Matrix4d {
    e11: f32,
    e12: f32,
    e13: f32,
    e14: f32,
    e21: f32,
    e22: f32,
    e23: f32,
    e24: f32,
    e31: f32,
    e32: f32,
    e33: f32,
    e34: f32,
    e41: f32,
    e42: f32,
    e43: f32,
    e44: f32
}

impl Matrix4d {
    pub fn new(e11: f32, e12: f32, e13: f32, e14: f32, e21: f32, e22: f32, e23: f32, e24: f32, e31: f32, e32: f32, e33: f32, e34: f32, e41: f32, e42: f32, e43: f32, e44: f32) -> Matrix4d {
        Matrix4d {
            e11,
            e12,
            e13,
            e14,
            e21,
            e22,
            e23,
            e24,
            e31,
            e32,
            e33,
            e34,
            e41,
            e42,
            e43,
            e44
        }
    }
    pub fn identity() -> Matrix4d {
        Matrix4d {
            e11: 1.0,
            e12: 0.0,
            e13: 0.0,
            e14: 0.0,
            e21: 0.0,
            e22: 1.0,
            e23: 0.0,
            e24: 0.0,
            e31: 0.0,
            e32: 0.0,
            e33: 1.0,
            e34: 0.0,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0
        }
    }

    //平行移動行列
    pub fn translate(x: f32, y: f32, z: f32) -> Matrix4d {
        Matrix4d {
            e11: 1.0,
            e12: 0.0,
            e13: 0.0,
            e14: x,
            e21: 0.0,
            e22: 1.0,
            e23: 0.0,
            e24: y,
            e31: 0.0,
            e32: 0.0,
            e33: 1.0,
            e34: z,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0
        }
    }

    //拡大行列
    pub fn scale(x: f32, y: f32, z: f32) -> Matrix4d {
        Matrix4d {
            e11: x,
            e12: 0.0,
            e13: 0.0,
            e14: 0.0,
            e21: 0.0,
            e22: y,
            e23: 0.0,
            e24: 0.0,
            e31: 0.0,
            e32: 0.0,
            e33: z,
            e34: 0.0,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0
        }
    }

    //回転行列
    pub fn rotate(roll: f32, pitch: f32, yaw: f32) -> Matrix4d {
        let sr = roll.sin();
        let cr = roll.cos();
        let sp = pitch.sin();
        let cp = pitch.cos();
        let sy = yaw.sin();
        let cy = yaw.cos();
        Matrix4d {
            e11: cy * cp,
            e12: cy * sp * sr - sy * cr,
            e13: cy * sp * cr + sy * sr,
            e14: 0.0,
            e21: sy * cp,
            e22: sy * sp * sr + cy * cr,
            e23: sy * sp * cr - cy * sr,
            e24: 0.0,
            e31: -sp,
            e32: cp * sr,
            e33: cp * cr,
            e34: 0.0,
            e41: 0.0,
            e42: 0.0,
            e43: 0.0,
            e44: 1.0
        }
    }

    pub fn perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Matrix4d {
        let f = 1.0 / (fovy * 0.5).tan();
        let zz = (zfar + znear) / (znear - zfar);
        let zw = (2.0 * zfar * znear) / (znear - zfar);
        Matrix4d {
            e11: f / aspect,
            e12: 0.0,
            e13: 0.0,
            e14: 0.0,
            e21: 0.0,
            e22: f,
            e23: 0.0,
            e24: 0.0,
            e31: 0.0,
            e32: 0.0,
            e33: zz,
            e34: -1.0,
            e41: 0.0,
            e42: 0.0,
            e43: zw,
            e44: 0.0
        }
    }

    pub fn look_at(eye: Vector3d, center: Vector3d, up: Vector3d) -> Matrix4d {
        let f = (center - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        Matrix4d {
            e11: s.x,
            e12: u.x,
            e13: -f.x,
            e14: 0.0,
            e21: s.y,
            e22: u.y,
            e23: -f.y,
            e24: 0.0,
            e31: s.z,
            e32: u.z,
            e33: -f.z,
            e34: 0.0,
            e41: -s.dot(eye),
            e42: -u.dot(eye),
            e43: f.dot(eye),
            e44: 1.0
        }
    }

    pub fn to_list(&self) -> [[f32; 4];4] {
        [[self.e11, self.e12, self.e13, self.e14],
        [self.e21, self.e22, self.e23, self.e24],
        [self.e31, self.e32, self.e33, self.e34],
        [self.e41, self.e42, self.e43, self.e44]]
    }
}