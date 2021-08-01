#[cfg(test)]
mod tests {
    use std::any::{Any, type_name};
    use crate::math_engine::Vector3;
    use crate::ecs::{Component, entity_manager};
    

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
    fn component_modify_test() -> Result<(), String>{
        let entity_manager = &mut *entity_manager::get_instance();
        let ent1 = entity_manager.create_entity();
        let ent2 = entity_manager.create_entity();
        
        // Modify the "Transform" component for entity 1
        let transform = entity_manager.assign::<Transform>(ent1)?;
        transform.vec3.x = 6.;
        transform.vec3.y = 0.;
        transform.vec3.z = 8.;
        
        // Check that "Transform" component was actually modified:
        let transform1 = entity_manager.get_component::<Transform>(ent1)?;
        assert_eq!(transform1.vec3.magnitude(), 10.);
        
        // Sanity check
        entity_manager.assign::<Transform>(ent2)?;
        let transform2 = entity_manager.get_component::<Transform>(ent2)?;
        assert_eq!(transform2.vec3.magnitude(), 0.);

        entity_manager.clear();
        Ok(())
    }

    #[test]
    fn component_assign_test(){
        let entity_manager = &mut *entity_manager::get_instance();
        let ent1 = entity_manager.create_entity();
        let ent2 = entity_manager.create_entity();
        let ent3 = entity_manager.create_entity();

        entity_manager.assign::<Transform>(ent1).unwrap();
        entity_manager.assign::<Transform>(ent2).unwrap();
        entity_manager.assign::<Transform>(ent3).unwrap();  
        entity_manager.assign::<Shape>(ent3).unwrap();

        match entity_manager.get_component::<Transform>(ent2){
            Ok(_) => (),
            Err(err_msg) => unreachable!("{}", err_msg)
        }
        match entity_manager.get_component::<Transform>(ent3){
            Ok(_) => (),
            Err(err_msg) => unreachable!("{}", err_msg)
        }
        match entity_manager.get_component::<Shape>(ent3){
            Ok(_) => (),
            Err(err_msg) => unreachable!("{}", err_msg)
        }
        match entity_manager.get_component::<Shape>(ent1){
            Ok(_) => unreachable!("ent1 shouldn't have \"Shape\" component"),
            Err(_) => ()
        }
        entity_manager.clear();
    }

    #[test]
    fn component_pool_size_test(){
        let entity_manager = &mut *entity_manager::get_instance();
        let ent1 = entity_manager.create_entity();
        let ent2 = entity_manager.create_entity();
        let ent3 = entity_manager.create_entity();

        match entity_manager.get_component_pool_size::<Transform>(){
            Ok(_) => unreachable!("Should be empty!"),
            Err(_) => ()
        };

        entity_manager.assign::<Transform>(ent1).expect("0");
        let pool_size = match entity_manager.get_component_pool_size::<Transform>(){
            Ok(size) => size,
            Err(err_msg) => unreachable!("{}", err_msg),
        };

        assert_eq!(pool_size, 1);

        entity_manager.assign::<Transform>(ent2).expect("1");
        entity_manager.assign::<Transform>(ent3).expect("2");

        let pool_size = match entity_manager.get_component_pool_size::<Transform>(){
            Ok(size) => size,
            Err(err_msg) => unreachable!("{}", err_msg),
        };

        assert_eq!(pool_size, 3);
        entity_manager.clear();
    }

    #[test]
    fn remove_component_test(){
        let entity_manager = &mut *entity_manager::get_instance();
        let ent1 = entity_manager.create_entity();
        let ent2 = entity_manager.create_entity();
        
        entity_manager.assign::<Transform>(ent1).unwrap();
        entity_manager.assign::<Transform>(ent2).unwrap();
        let result = entity_manager.get_component::<Transform>(ent1);
        match result{
            Ok(_) => (),
            Err(err_msg) => unreachable!("{}", err_msg),
        }
        
        entity_manager.remove::<Transform>(ent1).unwrap();
        let result = entity_manager.get_component::<Transform>(ent1);
        match result{
            Ok(_) => unreachable!(), 
            Err(err_msg) => assert_eq!(err_msg, String::from("The entity with ID 0 does not own this component!"))
        }
        entity_manager.clear();
    }

    #[test]
    fn remove_entity_test(){
        let entity_manager = &mut *entity_manager::get_instance();
        let ent1 = entity_manager.create_entity();
        
        entity_manager.assign::<Transform>(ent1).unwrap();
        let result = entity_manager.get_component::<Transform>(ent1);
        match result{
            Ok(_) => (),
            Err(err_msg) => unreachable!("{}", err_msg),
        }
        
        entity_manager.remove_entity(ent1).unwrap();
        let result = entity_manager.get_component::<Transform>(ent1);
        match result{
            Ok(_) => unreachable!(), 
            Err(err_msg) => assert_eq!(err_msg, format!("The component {} does not exist in this scene!", type_name::<Transform>()))
        }

        let result = entity_manager.remove_entity(ent1);
        match result{
            Ok(_) => unreachable!(), 
            Err(err_msg) => assert_eq!(err_msg, String::from("The entity with ID 0 does not exist!"))
        }
        entity_manager.clear();
    }
}