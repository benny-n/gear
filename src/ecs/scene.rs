use std::{any::type_name, collections::hash_map::Entry};

use super::{Component, Entity, EntityId, ComponentPool, ComponentTypeId, HashMap};

pub struct Scene{
    component_type_counter : i64,
    entity_id_counter : i64,
    entities : Vec<Entity>,
    component_name_to_type_id : HashMap<String, ComponentTypeId>,
    component_pools : HashMap<ComponentTypeId, ComponentPool>,
}
impl Scene{
    pub fn new() -> Self{
        Scene{
            component_type_counter : 0,
            entity_id_counter : 0,
            entities : Vec::new(),
            component_name_to_type_id : HashMap::new(),
            component_pools : HashMap::new(),
        }
    }
    pub fn create_entity(&mut self) -> EntityId{
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;  
        self.entities.push(Entity::new(id));
        id
    }
    pub fn get_component_type_id<T>(&mut self) -> ComponentTypeId{
        match self.component_name_to_type_id.get(type_name::<T>()){
            Some(component_id) => *component_id,
            None => {
                let component_id = self.component_type_counter;
                self.component_name_to_type_id.insert(String::from(type_name::<T>()), component_id);
                self.component_type_counter += 1;
                component_id
            }
        }         
    }

    pub fn get_component_pool_size<T>(&mut self) -> Result<usize, String>{
        let component_type_id = &self.get_component_type_id::<T>();
        match self.component_pools.get(component_type_id){
            Some(pool) => Ok(pool.len()),
            None => Err(String::from("Pool does not exist for this component")),
        }    
    }

    pub fn get_component<T: Component>(&mut self, entity_id: EntityId) -> Result<&mut dyn Component, String>{
        let component_id = &self.get_component_type_id::<T>();

        let component_pool = self.component_pools.get_mut(component_id);
        match component_pool{
            Some(pool) => {
                match pool.get_mut(&entity_id){
                    Some(component) => Ok(&mut **component),
                    None => {
                        let error_msg = 
                            format!("The entity with ID {} does not own this component!", entity_id);
                        Err(error_msg) 
                    }
                }    
            }
            None => {
                let error_msg = 
                    format!("The component does not exist in this scene!");
                Err(error_msg)
            }
        }
    }

    //TODO implement remove component
    pub fn remove<T: 'static + Component>(&mut self, _entity_id: EntityId) -> Result<(), String>{
        todo!();
    }

    pub fn assign<T: 'static + Component + Default>(&mut self, entity_id: EntityId) -> Result<&mut dyn Component, String>{
        let component_type_id = self.get_component_type_id::<T>();

        match self.component_pools
            .entry(component_type_id)
            .or_insert_with(HashMap::new)
            .entry(entity_id){
                Entry::Occupied(_) => 
                    Err(format!("Component {} already exists for entity {}", type_name::<T>(), entity_id)),
                Entry::Vacant(_) => {
                    self.component_pools
                        .entry(component_type_id)
                        .or_insert_with(HashMap::new)
                        .insert(entity_id,Box::new(T::default()));  
                    Ok(self.get_component::<T>(entity_id).unwrap())
            },   
        }
    }
}