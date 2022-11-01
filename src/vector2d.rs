use crate::Vertex;

#[derive(Copy, Clone, Debug)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn new(x: f32, y: f32) -> Vector2d {
        Vector2d { x, y }
    }

    pub fn zero() -> Vector2d {
        Vector2d { x: 0.0, y: 0.0 }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }

    pub fn dot(&self, other: &Vector2d) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Vector2d) -> f32 {
        self.x * other.y - self.y * other.x
    }

    pub fn multiply_scalar(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn rotate(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        let x = self.x * cos - self.y * sin;
        let y = self.x * sin + self.y * cos;
        self.x = x;
        self.y = y;
    }

    pub fn rotate_absolute(&mut self, angle: f32) {
        let len = self.length();
        self.x = len * angle.cos();
        self.y = len * angle.sin();
    }

    pub fn reflect_x(&mut self) {
        self.x = -self.x;
    }

    pub fn reflect_y(&mut self) {
        self.y = -self.y;
    }

    pub fn to_list(&self) -> [f32; 2] {
        return [self.x, self.y]
    }
}

impl std::ops::Add for Vector2d {
    type Output = Vector2d;

    fn add(self, other: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Vector2d {
    fn add_assign(&mut self, other: Vector2d) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Sub for Vector2d {
    type Output = Vector2d;

    fn sub(self, other: Vector2d) -> Vector2d {
        Vector2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign for Vector2d {
    fn sub_assign(&mut self, other: Vector2d) {
        self.x -= other.x;
        self.y -= other.y;
    }
}