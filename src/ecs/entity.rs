use super::EntityId;


#[derive(Hash, Debug)]
pub struct Entity{
    id : EntityId,
}
impl Entity{
    pub fn new(id: EntityId,) -> Entity{
        Entity{
            id
        }
    }
}
impl PartialEq for Entity{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Entity{}