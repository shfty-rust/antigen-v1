use antigen::{
    components::EventQueue,
    core::events::AntigenInputEvent,
    entity_component_system::system_interface::SystemInterface,
    entity_component_system::ComponentStorage,
    entity_component_system::EntityComponentDirectory,
    entity_component_system::{SystemError, SystemTrait},
};

#[derive(Debug)]
pub struct QuitKey {
    key: antigen::core::keyboard::Key,
}

impl QuitKey {
    pub fn new(key: antigen::core::keyboard::Key) -> Self {
        QuitKey { key }
    }
}

impl<CS, CD> SystemTrait<CS, CD> for QuitKey
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueue<AntigenInputEvent>>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            let event_queue: &Vec<AntigenInputEvent> =
                db.get_entity_component::<EventQueue<AntigenInputEvent>>(event_queue_entity)?;

            for event in event_queue {
                if let AntigenInputEvent::KeyPress { key_code } = event {
                    if *key_code == self.key {
                        return Err(SystemError::Quit);
                    }
                }
            }
        }

        Ok(())
    }
}
