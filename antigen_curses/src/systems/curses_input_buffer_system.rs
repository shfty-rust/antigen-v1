use antigen::{
    components::WindowComponent,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory,
        SystemDebugTrait, SystemError, SystemTrait,
    },
};

use crate::{CursesEventQueueComponent, CursesWindowComponent};

/// Reads input from a pancurses window and pushes it into an event queue
#[derive(Debug)]
pub struct CursesInputBufferSystem;

impl<CS, CD> SystemTrait<CS, CD> for CursesInputBufferSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Fetch event queue entity
        let event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<CursesEventQueueComponent>(entity_id)
                });

        if let Some(event_queue_entity) = event_queue_entity {
            // If event queue exists, fetch the window entry we'll be reading input from
            let window_entity =
                db.entity_component_directory
                    .get_entity_by_predicate(|entity_id| {
                        db.entity_component_directory
                            .entity_has_component::<WindowComponent>(entity_id)
                            && db
                                .entity_component_directory
                                .entity_has_component::<CursesWindowComponent>(entity_id)
                    });

            if let Some(entity_id) = window_entity {
                if let Some(window) = db
                    .get_entity_component::<CursesWindowComponent>(entity_id)?
                    .get_window()
                {
                    if let Some(input) = window.getch() {
                        // Fetch the entity queue component and push inputs into it
                        let event_queue_component = db
                            .get_entity_component_mut::<CursesEventQueueComponent>(
                                event_queue_entity,
                            )?;

                        event_queue_component.push_event(input);
                    }
                }
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for CursesInputBufferSystem {
    fn get_name() -> &'static str {
        "Curses Input Buffer"
    }
}