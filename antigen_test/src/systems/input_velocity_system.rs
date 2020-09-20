use crate::components::pancurses_input_buffer_component::PancursesInputBufferComponent;
use antigen::{
    components::VelocityComponent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::SystemDebugTrait,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct InputVelocitySystem;

impl InputVelocitySystem {
    pub fn new() -> Self {
        InputVelocitySystem
    }
}

impl<CS, CD> SystemTrait<CS, CD> for InputVelocitySystem
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
                    .entity_has_component::<PancursesInputBufferComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<VelocityComponent>(entity_id)
            });

        for entity_id in entities {
            let pancurses_input_buffer_component =
                db.get_entity_component::<PancursesInputBufferComponent>(entity_id)?;

            let mut move_input: IVector2 = IVector2(0, 0);
            for input in pancurses_input_buffer_component.get_inputs() {
                match input {
                    pancurses::Input::KeyLeft => move_input.0 -= 1,
                    pancurses::Input::KeyRight => move_input.0 += 1,
                    pancurses::Input::KeyUp => move_input.1 -= 1,
                    pancurses::Input::KeyDown => move_input.1 += 1,
                    _ => (),
                }
            }

            move_input.0 = std::cmp::min(std::cmp::max(move_input.0, -1), 1);
            move_input.1 = std::cmp::min(std::cmp::max(move_input.1, -1), 1);

            db.get_entity_component_mut::<VelocityComponent>(entity_id)?
                .set_velocity(move_input);
        }

        Ok(())
    }
}

impl SystemDebugTrait for InputVelocitySystem {
    fn get_name() -> &'static str {
        "Input Velocity"
    }
}
