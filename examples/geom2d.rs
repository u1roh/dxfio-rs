#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Pos {
    pub x: f64,
    pub y: f64,
}
impl From<&[f64; 3]> for Pos {
    fn from(src: &[f64; 3]) -> Self {
        Self {
            x: src[0],
            y: src[1],
        }
    }
}
impl From<Pos> for [f64; 3] {
    fn from(src: Pos) -> Self {
        [src.x, src.y, 0.0]
    }
}
impl std::ops::Add<Vec> for Pos {
    type Output = Self;
    fn add(self, v: Vec) -> Self {
        Self {
            x: self.x + v.x,
            y: self.y + v.y,
        }
    }
}
impl std::ops::Sub for Pos {
    type Output = Vec;
    fn sub(self, p: Self) -> Vec {
        Vec {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec {
    pub x: f64,
    pub y: f64,
}
impl std::ops::Mul<Vec> for f64 {
    type Output = Vec;
    fn mul(self, v: Vec) -> Self::Output {
        Vec {
            x: self * v.x,
            y: self * v.y,
        }
    }
}
impl std::ops::Div<f64> for Vec {
    type Output = Self;
    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}
impl Vec {
    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
    pub fn norm2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn norm(&self) -> f64 {
        self.norm2().sqrt()
    }
    pub fn normalize(&self) -> Option<UnitVec> {
        let v = *self / self.norm();
        v.is_finite().then(|| UnitVec(v))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct UnitVec(Vec);
impl std::ops::Deref for UnitVec {
    type Target = Vec;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl UnitVec {
    pub fn of_angle(theta: f64) -> Self {
        Self(Vec {
            x: theta.cos(),
            y: theta.sin(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Matrix([[f64; 2]; 2]);
impl std::ops::Deref for Matrix {
    type Target = [[f64; 2]; 2];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::Mul<Vec> for Matrix {
    type Output = Vec;
    fn mul(self, v: Vec) -> Vec {
        Vec {
            x: self[0][0] * v.x + self[0][1] * v.y,
            y: self[1][0] * v.x + self[1][1] * v.y,
        }
    }
}
impl Matrix {
    pub fn new(elms: [[f64; 2]; 2]) -> Self {
        Self(elms)
    }
    pub fn of_columns(v1: &Vec, v2: &Vec) -> Self {
        Self([[v1.x, v2.x], [v1.y, v2.y]])
    }
    pub fn det(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
    pub fn is_finite(&self) -> bool {
        self.iter().flatten().all(|e| e.is_finite())
    }
    pub fn inverse(&self) -> Option<Self> {
        let det = self.det();
        let inv = Self::new([
            [self[1][1] / det, -self[0][1] / det],
            [-self[1][0] / det, self[0][0] / det],
        ]);
        inv.is_finite().then(|| inv)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Line {
    pub p: Pos,
    pub v: UnitVec,
}
impl Line {
    pub fn at(&self, t: f64) -> Pos {
        self.p + t * (*self.v)
    }
    pub fn intersection(&self, other: &Self) -> Option<[f64; 2]> {
        Matrix::of_columns(&self.v, &other.v).inverse().map(|inv| {
            let t = inv * (other.p - self.p);
            [t.x, -t.y]
        })
    }
    pub fn intersection_pos(&self, other: &Self) -> Option<Pos> {
        self.intersection(other).map(|[t, _]| self.at(t))
    }
}
