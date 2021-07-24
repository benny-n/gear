#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::ecs::scene::Scene;
    use crate::math_engine::Vector3;
    use crate::ecs::Component;

    #[derive(Default, Component)]
    struct Shape {
        #[allow(dead_code)]
        vec3 : Vector3
    }
    #[derive(Default, Component)]
    struct Transform {
        #[allow(dead_code)]
        pub vec3 : Vector3
    }

    #[test]
    fn get_id_api_test(){
        let mut scene = Scene::new();
        let id1 = scene.get_component_type_id::<Transform>();
        let id2 = scene.get_component_type_id::<Shape>();
        let id3 = scene.get_component_type_id::<Transform>();
        let id4 = scene.get_component_type_id::<Transform>();
        let id5 = scene.get_component_type_id::<Shape>();
        
        assert_eq!(id1, id3);
        assert_ne!(id1, id2);
        assert_ne!(id3, id5);
        assert_ne!(id5, id4);
        assert_eq!(id5, id2);
        assert_eq!(id3, id4);
    }

    #[test]
    fn component_assign_test(){
        let mut scene = Scene::new();
        let ent1 = scene.create_entity();
        let ent2 = scene.create_entity();
        let ent3 = scene.create_entity();

        scene.assign::<Transform>(ent1).unwrap();
        scene.assign::<Transform>(ent2).unwrap();
        scene.assign::<Transform>(ent3).unwrap();
    
        scene.assign::<Shape>(ent3).unwrap();

        match scene.get_component::<Transform>(ent1){
            Ok(component) => {
                let transform: &mut Transform = match component.as_mut_any().downcast_mut::<Transform>() {
                    Some(transform) => transform,
                    None => panic!("&component isn't a Transform!"),
                };
                transform.vec3.x = 6.;
                transform.vec3.y = 0.;
                transform.vec3.z = 8.;
            }
            Err(err_msg) => panic!("{}", err_msg)
        }

        // Check that "Transform" component was actually modified:
        match scene.get_component::<Transform>(ent1){
            Ok(component) => {
                let transform: &Transform = match component.as_any().downcast_ref::<Transform>() {
                    Some(transform) => transform,
                    None => panic!("&component isn't a Transform!"),
                };
                assert_eq!(transform.vec3.magnitude(), 10.)
            }
            Err(err_msg) => panic!("{}", err_msg)
        }

        match scene.get_component::<Transform>(ent2){
            Ok(_) => (),
            Err(err_msg) => panic!("{}", err_msg)
        }
        match scene.get_component::<Transform>(ent3){
            Ok(_) => (),
            Err(err_msg) => panic!("{}", err_msg)
        }
        match scene.get_component::<Shape>(ent3){
            Ok(_) => (),
            Err(err_msg) => panic!("{}", err_msg)
        }
        match scene.get_component::<Shape>(ent1){
            Ok(_) => panic!("ent1 shouldn't have \"Shape\" component"),
            Err(_) => ()
        }
    }

    #[test]
    fn component_pool_size_test(){
        let mut scene = Scene::new();
        let ent1 = scene.create_entity();
        let ent2 = scene.create_entity();
        let ent3 = scene.create_entity();

        match scene.get_component_pool_size::<Transform>(){
            Ok(_) => panic!("Should be empty!"),
            Err(_) => ()
        };

        scene.assign::<Transform>(ent1).expect("0");
        let pool_size = match scene.get_component_pool_size::<Transform>(){
            Ok(size) => size,
            Err(err_msg) => panic!("{}", err_msg),
        };

        assert_eq!(pool_size, 1);

        scene.assign::<Transform>(ent2).expect("1");
        scene.assign::<Transform>(ent3).expect("2");

        let pool_size = match scene.get_component_pool_size::<Transform>(){
            Ok(size) => size,
            Err(err_msg) => panic!("{}", err_msg),
        };

        assert_eq!(pool_size, 3);
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn remove_component_test(){
        let mut scene = Scene::new();
        let ent1 = scene.create_entity();
        scene.assign::<Transform>(ent1).expect("0");
        scene.remove::<Transform>(ent1).unwrap(); // Should panic here
    }
}