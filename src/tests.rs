mod math_engine;

#[cfg(test)]
mod tests {
    use crate::math_engine::vector::vector::Vector3;
    use crate::math_engine::matrix::matrix::Matrix4;
    
    #[test]
    fn vector_magnitude_test(){
        let vector3 = Vector3::new(8., 0., 6.);
        assert_eq!(vector3.magnitude(), 10.);
    }
    

    /* This test calls a function that isn't implemented yet, and therefore it should panic. 
    However, the test should still pass due to #[should_panic] annotation. */
    #[test]
    #[should_panic(expected = "not implemented: inverse for 4x4 matrix")]
    fn matrix_inverse_test(){
        let matrix4 = Matrix4::new([[0f64; 4]; 4]);
        matrix4.inverse();
    }
}


