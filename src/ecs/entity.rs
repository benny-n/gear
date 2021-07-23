use super::{ComponentTypeId, EntityId};


#[derive(Hash, Debug)]
pub struct Entity{
    id : EntityId,
    components: Vec<ComponentTypeId>,
}
impl Entity{
    pub fn new(id: EntityId,) -> Entity{
        Entity{
            components: Vec::new(),
            id
        }
    }
    // pub fn id(&self) -> EntityId{
    //     self.id
    // }
    // pub fn components(&self) -> &Vec<ComponentTypeId>{
    //     &self.components
    // }
    pub fn add_component(&mut self, component_id : ComponentTypeId){
        self.components.push(component_id);
    }
    // pub fn get_component_by_id(&self) -> (){

    // }
}
impl PartialEq for Entity{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Entity{}