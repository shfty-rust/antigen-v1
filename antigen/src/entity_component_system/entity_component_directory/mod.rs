mod single_threaded_directory;
use std::collections::HashMap;

pub use single_threaded_directory::SingleThreadedDirectory;

use crate::entity_component_system::{ComponentDataID, ComponentID, ComponentTrait, EntityID};

pub trait EntityComponentDirectory {
    // CREATE
    fn create_entity(&mut self) -> Result<EntityID, String>;

    // INSERT
    fn insert_component<T: ComponentTrait + 'static>(&mut self) -> Result<ComponentID, String>;

    fn insert_entity_component<T>(
        &mut self,
        entity_id: &EntityID,
        component_data_id: ComponentDataID,
    ) -> Result<ComponentDataID, String>
    where
        T: ComponentTrait + 'static;

    // DESTROY
    fn destroy_component<T: ComponentTrait + 'static>(&mut self) -> Result<(), String>;

    fn destroy_entity(&mut self, entity_id: EntityID) -> Result<(), String>;

    fn destroy_entity_component(
        &mut self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<(), String>;

    // EXIST
    fn is_valid_entity(&self, entity_id: &EntityID) -> bool;
    fn is_valid_component<T: ComponentTrait + 'static>(&self) -> bool;

    // GET
    fn get_entity_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Option<EntityID>;
    fn get_entities_by_predicate(&self, predicate: impl Fn(&EntityID) -> bool) -> Vec<EntityID>;

    fn get_components_by_predicate(
        &self,
        predicate: impl Fn(&ComponentID) -> bool,
    ) -> Vec<ComponentID>;

    fn entity_has_component_by_id(&self, entity_id: &EntityID, component_id: &ComponentID) -> bool;

    fn get_entity_component_data_id(
        &self,
        entity_id: &EntityID,
        component_id: &ComponentID,
    ) -> Result<ComponentDataID, String>;

    fn get_entity_component_data(
        &self,
        entity_id: &EntityID,
    ) -> Result<HashMap<ComponentID, ComponentDataID>, String>;

    // Derived methods
    fn entity_has_component<T: ComponentTrait + 'static>(&self, entity_id: &EntityID) -> bool {
        self.entity_has_component_by_id(entity_id, &ComponentID::get::<T>())
    }
}
