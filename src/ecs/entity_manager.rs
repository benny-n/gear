use std::{any::{TypeId, type_name}, collections::hash_map::Entry, sync::{Mutex, MutexGuard}};

use super::{Component, Entity, EntityId, ComponentPool, HashMap};
use lazy_static::{lazy_static};

lazy_static! {
    pub static ref ENTITY_MANAGER: Mutex<EntityManager> = {
        let instance = Mutex::new(EntityManager::new());
        instance
    };
}
pub fn get_instance() -> MutexGuard<'static, EntityManager>{
    ENTITY_MANAGER.lock().unwrap()
}
pub struct EntityManager{
    entity_id_counter : u64,
    entities : HashMap<EntityId, Entity>,
    component_pools : HashMap<TypeId, ComponentPool>,
}
unsafe impl Sync for EntityManager{}
impl EntityManager{
    fn new() -> Self{
        EntityManager{
            entity_id_counter : 0,
            entities : HashMap::new(),
            component_pools : HashMap::new(),
        }
    }

    pub fn clear(&mut self){
        self.entity_id_counter = 0;
        self.entities = HashMap::new();
        self.component_pools = HashMap::new();
    }

    pub fn create_entity(&mut self) -> EntityId{
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;  
        self.entities.insert(id, Entity::new(id));
        id
    }
    pub fn remove_entity(&mut self, entity_id: EntityId) -> Result<(), String>{
        let result = self.entities.remove(&entity_id);
        match result{
            Some(_) => {
                for pool in self.component_pools.values_mut(){
                    pool.remove_entry(&entity_id);
                }
                self.component_pools.retain(|_, pool|{
                    !pool.is_empty()
                });
                Ok(())
            },
            None => Err(format!("The entity with ID {} does not exist!", entity_id)),
        }
    }

    pub fn get_component_pool_size<T: 'static>(&self) -> Result<usize, String>{
        match self.component_pools.get(&TypeId::of::<T>()){
            Some(pool) => Ok(pool.len()),
            None => Err(String::from("Pool does not exist for this component")),
        }    
    }

    fn get_component_from_pool<'a>(entity_id : EntityId, pool : &'a mut ComponentPool) -> Result<&'a mut dyn Component, String>{
        match pool.get_mut(&entity_id){
            Some(component) => Ok(&mut **component),
            None => Err(format!("The entity with ID {} does not own this component!", entity_id)), 
        }       
    }

    pub fn get_component<T: 'static + Component>(&mut self, entity_id: EntityId) -> Result<&mut T, String>{
        let component_pool = self.component_pools.get_mut(&TypeId::of::<T>());

        match component_pool{
            Some(pool) => {
                let component = EntityManager::get_component_from_pool(entity_id, pool);
                Ok(component?.as_mut_any().downcast_mut::<T>().unwrap())
            },
            None => Err(format!("The component {} does not exist in this scene!", type_name::<T>())),
        }
    }

    pub fn remove<T: 'static + Component>(&mut self, entity_id: EntityId) -> Result<(), String>{
        let component_type_id = &TypeId::of::<T>();

        match self.get_component::<T>(entity_id){
            Ok(_) => {
                let pool = self.component_pools
                .get_mut(component_type_id)
                .unwrap();
                pool.remove_entry(&entity_id);
                if pool.is_empty(){
                    self.component_pools.remove_entry(component_type_id);
                }
                Ok(())
            },
            Err(err_msg) => Err(err_msg),
        }
    }

    pub fn assign<T: 'static + Send + Component + Default>(&mut self, entity_id: EntityId) -> Result<&mut T, String>{

        let component_pool =  self.component_pools
            .entry(TypeId::of::<T>())
            .or_insert_with(HashMap::new);

        match component_pool.entry(entity_id){
            
                Entry::Occupied(_) => 
                    Err(format!("Component {} already exists for entity {}", type_name::<T>(), entity_id)),

                Entry::Vacant(entry) => {
                    entry.insert(Box::new(T::default()));  
                    Ok(self.get_component::<T>(entity_id).unwrap())
            },   
        }
    }
}