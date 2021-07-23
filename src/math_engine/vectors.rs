#[derive (Debug, PartialEq, Default)]
pub struct Vector3{
    pub x : f64,
    pub y : f64,
    pub z : f64,
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

#[derive (Debug, PartialEq, Default)]
pub struct Vector2{
    pub x : f64,
    pub y : f64,
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
    pub fn distance(first_vector: &Vector2, second_vector: &Vector2) -> f64 {
        let distance : f64;
        let x_delta_squared: f64 = (first_vector.x - second_vector.x).powi(2);
        let y_delta_squared : f64 = (first_vector.y - second_vector.y).powi(2);
        distance = (x_delta_squared + y_delta_squared).sqrt();
        distance
    }
}
