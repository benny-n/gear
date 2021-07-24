#[cfg(test)]
pub mod tests {
    use crate::math_engine::Matrix4;
    
    /* This test calls a function that isn't implemented yet, and therefore it should panic. 
    However, the test should still pass due to #[should_panic] annotation. */
    #[test]
    #[should_panic(expected = "not implemented: inverse for 4x4 matrix")]
    fn matrix_inverse_test(){
        let matrix4 = Matrix4::new([[0f64; 4]; 4]);
        matrix4.inverse();
    }
}