use crate::components::{
    pancurses_input_axis_component::PancursesInputAxisComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
};
use antigen::{
    components::IntRangeComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::{SystemError, SystemTrait},
entity_component_system::SystemDebugTrait};

#[derive(Debug)]
pub struct PancursesInputAxisSystem;

impl PancursesInputAxisSystem {
    pub fn new() -> Self {
        PancursesInputAxisSystem
    }
}

impl<CS, CD> SystemTrait<CS, CD> for PancursesInputAxisSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let entities = db
            .entity_component_directory
            .get_entities_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PancursesInputAxisComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<PancursesInputBufferComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<IntRangeComponent>(entity_id)
            });

        for entity_id in entities {
            let pancurses_prev_next_input_component =
                db.get_entity_component::<PancursesInputAxisComponent>(entity_id)?;
            let (prev_input, next_input) = (
                pancurses_prev_next_input_component.get_negative_input(),
                pancurses_prev_next_input_component.get_positive_input(),
            );

            let pancurses_input_buffer_component =
                db.get_entity_component_mut::<PancursesInputBufferComponent>(entity_id)?;

            let mut offset: i64 = 0;

            for input in pancurses_input_buffer_component.get_inputs() {
                let input = input;

                if input == prev_input {
                    offset -= 1;
                } else if input == next_input {
                    offset += 1;
                } else {
                    return Ok(());
                }
            }

            let ui_tab_input_component =
                db.get_entity_component_mut::<IntRangeComponent>(entity_id)?;

            ui_tab_input_component.set_index(ui_tab_input_component.get_index() + offset);
        }

        Ok(())
    }
}

impl SystemDebugTrait for PancursesInputAxisSystem {
    fn get_name() -> &'static str {
        "Pancurses Input Axis"
    }
}