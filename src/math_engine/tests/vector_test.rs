#[cfg(test)]
pub mod tests {
    use crate::math_engine::Vector3;
    use crate::math_engine::Vector2;
    
    #[test]
    fn vector_magnitude_test(){
        let vector3 = Vector3::new(8., 0., 6.);
        assert_eq!(vector3.magnitude(), 10.);
        
        let vector2 = Vector2::new(3., 4.);
        assert_eq!(vector2.magnitude(), 5.);
    }

    #[test]
    fn vector_distance_test(){
        let vector2 = Vector2::new(8., 4.);
        
        let other_vector2 = Vector2::new(3., 4.);
        assert_eq!(Vector2::distance(&vector2, &other_vector2), 5.);
    }
}