#[derive (Debug, PartialEq)]
pub struct Matrix4{
    #[allow(dead_code)] // TODO: this is here only to silence compiler warnings, remove this later
    mat : [[f64; 4]; 4],
}
impl Matrix4{
    pub fn new(mat: [[f64; 4]; 4]) -> Matrix4{
        Matrix4{
            mat,
        }
    }
    pub fn inverse(&self) -> (){
        unimplemented!("inverse for 4x4 matrix")
    }
}

