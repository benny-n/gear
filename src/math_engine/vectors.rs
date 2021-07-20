pub struct Vector3{
    x : f64,
    y : f64,
    z : f64,
}
impl Vector3{
    pub fn new(x: f64, y: f64, z: f64) -> Vector3{
        Vector3{
            x, y, z,
        }
    }
    pub fn magnitude(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

pub struct Vector2{
    x : f64,
    y : f64,
}
impl Vector2{
    pub fn new(x: f64, y: f64,) -> Vector2{
        Vector2{
            x, y,
        }
    }
    pub fn magnitude(&self) -> f64{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}