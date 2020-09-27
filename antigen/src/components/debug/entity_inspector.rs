use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone)]
pub struct EntityInspector;

impl ComponentDebugTrait for EntityInspector {
    fn get_name() -> String {
        "Entity Inspector".into()
    }

    fn get_description() -> String {
        "Tag component for entity inspector".into()
    }
}
