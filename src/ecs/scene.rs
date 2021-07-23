use once_cell::sync::OnceCell;
use generic_static::StaticTypeMap;
use super::{Component, Entity, EntityId, ComponentPool, ComponentTypeId, HashMap};

pub struct Scene{
    component_type_counter : i64,
    entity_id_counter : i64,
    entities : Vec<Entity>,
    component_pools : HashMap<ComponentTypeId, ComponentPool>,
}
impl Scene{
    pub fn new() -> Self{
        Scene{
            component_type_counter : 0,
            entity_id_counter : 0,
            entities : Vec::new(),
            component_pools : HashMap::new(),
        }
    }
    pub fn create_entity(&mut self) -> EntityId{
        let id = self.entity_id_counter;
        self.entity_id_counter += 1;  
        self.entities.push(Entity::new(id));
        id
    }

    pub fn get_component_type_id<T: 'static>(&mut self) -> ComponentTypeId{ // T is bound to 'static
        static GENERATE_ID: OnceCell<StaticTypeMap<i64>> = OnceCell::new();
        let type_map = GENERATE_ID.get_or_init(|| StaticTypeMap::new());

        *type_map.call_once::<T, _>(||{
            self.component_type_counter += 1;
            self.component_type_counter
        })
    }

    pub fn get_component_pool<T: 'static>(&mut self) -> Result<&ComponentPool, &str>{
        let component_type_id = self.get_component_type_id::<T>();
        match self.component_pools.get(&component_type_id){
            Some(pool) => Ok(pool),
            None => Err("Pool does not exist for this component"),
        }    
    }

    //TODO implement remove component

    pub fn get_component<T: 'static + Component>(&mut self, entity_id: EntityId) -> Result<&mut dyn Component, String>{
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

    pub fn assign<T: 'static +  Component + Default>(&mut self, entity_id: EntityId) -> (){
        let component_type_id = self.get_component_type_id::<T>();
    
        self.component_pools
            .entry(component_type_id)
            .or_insert_with(HashMap::new)
            .insert(entity_id,Box::new(T::default()));  
        
        let mut entity = Entity::new(entity_id);
        entity.add_component(component_type_id);
        self.entities.push(entity);
    }
}