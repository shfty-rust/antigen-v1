use crate::{
    components::ComponentDebugComponent, components::DebugComponentListComponent,
    components::DebugExcludeComponent, components::EntityInspectorComponent,
    components::IntRangeComponent, entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage, entity_component_system::EntityComponentDirectory,
    entity_component_system::EntityID, entity_component_system::SystemDebugTrait,
};
use crate::{
    components::StringListComponent,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct ComponentDebugSystem;

impl<CS, CD> SystemTrait<CS, CD> for ComponentDebugSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let mut debug_entities: Vec<EntityID> = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                !db.entity_component_directory
                    .entity_has_component::<DebugExcludeComponent>(entity_id)
            });
        debug_entities.sort();

        // Populate entity components list
        let entity_inspector_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EntityInspectorComponent>(entity_id)
                });

        // Populate strings for debug component list entities
        let debug_component_list_entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<DebugComponentListComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<StringListComponent>(entity_id)
            });

        if let Some(entity_inspector_entity) = entity_inspector_entity {
            let int_range_component =
                db.get_entity_component::<IntRangeComponent>(entity_inspector_entity)?;

            if let Some(inspected_entity) =
                debug_entities.get(int_range_component.get_index() as usize)
            {
                let component_debug_entity =
                    db.entity_component_directory
                        .get_entity_by_predicate(|entity_id| {
                            db.entity_component_directory
                                .entity_has_component::<ComponentDebugComponent>(entity_id)
                        });

                if let Some(component_debug_entity) = component_debug_entity {
                    let mut components =
                        db.entity_component_directory
                            .get_components_by_predicate(|component_id| {
                                db.entity_component_directory
                                    .entity_has_component_by_id(inspected_entity, component_id)
                            });

                    let component_debug_component =
                        db.get_entity_component::<ComponentDebugComponent>(component_debug_entity)?;

                    components.sort_by(|lhs, rhs| {
                        let lhs_label = component_debug_component.get_label(lhs);
                        let rhs_label = component_debug_component.get_label(rhs);

                        lhs_label.cmp(&rhs_label)
                    });

                    let component_strings: Vec<String> = components
                        .iter()
                        .map(|component_id| component_debug_component.get_label(component_id))
                        .collect();

                    for entity_id in debug_component_list_entities {
                        db.get_entity_component_mut::<StringListComponent>(entity_id)?
                            .set_data(component_strings.clone());
                    }
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for ComponentDebugSystem {
    fn get_name() -> &'static str {
        "Component Debug"
    }
}