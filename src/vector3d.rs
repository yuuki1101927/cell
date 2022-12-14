

#[derive(Copy, Clone)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d {
            x,
            y,
            z,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vector3d {
        let length = self.length();
        Vector3d {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn cross(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn dot(self, other: Vector3d) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn scale(self, scalar: f32) -> Vector3d {
        Vector3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    //真上から見た二次元ベクトル
    pub fn bird_view_z(self) -> Vector3d {
        Vector3d {
            x: self.x,
            y: 0.0,
            z: self.z,
        }
    }

    pub fn to_list(&self) -> [f32; 3] {
        return [self.x, self.y, self.z]
    }
}

impl std::ops::Add for Vector3d {
    type Output = Vector3d;

    fn add(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vector3d {
    fn add_assign(&mut self, other: Vector3d) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Vector3d {
    fn sub_assign(&mut self, other: Vector3d) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}