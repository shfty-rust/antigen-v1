use crate::entity_component_system::ComponentDebugTrait;

#[derive(Debug, Default, Clone)]
pub struct DebugComponentDataList;

impl ComponentDebugTrait for DebugComponentDataList {
    fn get_name() -> String {
        "Debug Entity Component List".into()
    }

    fn get_description() -> String {
        "Tag component for debug entity component list".into()
    }
}
