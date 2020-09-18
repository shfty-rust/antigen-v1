use std::collections::HashMap;

use crate::{
    components::local_mouse_position_component::LocalMousePositionComponent,
    components::{
        control_component::ControlComponent, list_component::ListComponent,
        pancurses_color_pair_component::PancursesColorPairComponent,
    },
    pancurses_color::PancursesColor,
    pancurses_color::PancursesColorPair,
};
use antigen::{
    components::DebugExcludeComponent,
    components::GlobalPositionComponent,
    components::IntRangeComponent,
    components::ParentEntityComponent,
    components::PositionComponent,
    components::SizeComponent,
    components::StringComponent,
    components::StringListComponent,
    entity_component_system::create_entity,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::get_entity_component,
    entity_component_system::get_entity_component_mut,
    entity_component_system::insert_entity_component,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::IVector2,
};

#[derive(Debug)]
pub struct ListSystem {
    // Maps list control entities -> string entities -> strings
    list_string_entities: HashMap<EntityID, Vec<EntityID>>,
}

impl ListSystem {
    pub fn new() -> Self {
        ListSystem {
            list_string_entities: HashMap::new(),
        }
    }
}

impl<CS, CD> SystemTrait<CS, CD> for ListSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run<'a>(&mut self, db: &'a mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let list_control_entities = db.entity_component_directory.get_entities_by_predicate(|entity_id| {
            db.entity_component_directory.entity_has_component::<ListComponent>(entity_id)
                && db.entity_component_directory.entity_has_component::<PositionComponent>(entity_id)
                && db.entity_component_directory.entity_has_component::<SizeComponent>(entity_id)
                && db.entity_component_directory.entity_has_component::<ParentEntityComponent>(entity_id)
        });

        for list_control_entity in list_control_entities {
            let (string_list_entity, list_index_entity) =
                match get_entity_component::<CS, CD, ListComponent>(
                    &db.component_storage,
                    &db.entity_component_directory,
                    list_control_entity,
                ) {
                    Ok(pancurses_list_control_component) => (
                        pancurses_list_control_component.get_string_list_entity(),
                        pancurses_list_control_component.get_list_index_entity(),
                    ),
                    Err(err) => return Err(err.into()),
                };

            if let Some(string_list_entity) = string_list_entity {
                let IVector2(width, height) = match get_entity_component::<CS, CD, SizeComponent>(
                    &db.component_storage,
                    &db.entity_component_directory,
                    list_control_entity,
                ) {
                    Ok(size_component) => size_component.get_size(),
                    Err(err) => return Err(err.into()),
                };

                // If we have a string list entity, fetch its strings
                let string_list: Vec<Vec<String>> =
                    get_entity_component::<CS, CD, StringListComponent>(
                        &db.component_storage,
                        &db.entity_component_directory,
                        string_list_entity,
                    )?
                    .get_data()
                    .iter()
                    .map(|string| {
                        let substrings: Vec<String> = string
                            .split('\n')
                            .map(|string| {
                                &string[..std::cmp::min(width, string.len() as i64) as usize]
                            })
                            .map(std::string::ToString::to_string)
                            .collect();
                        substrings
                    })
                    .collect();

                let string_count: usize = string_list.iter().map(|strings| strings.len()).sum();

                if self
                    .list_string_entities
                    .get(&list_control_entity)
                    .is_none()
                {
                    self.list_string_entities
                        .insert(list_control_entity, Vec::new());
                }

                let string_entities = match self.list_string_entities.get_mut(&list_control_entity)
                {
                    Some(string_entities) => string_entities,
                    None => {
                        return Err(format!(
                            "Failed to get list string entities for list control entity {}",
                            list_control_entity
                        )
                        .into())
                    }
                };

                while string_entities.len() < string_count {
                    let string_entity = create_entity(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        &mut db.callback_manager,
                        Some("List String Entity"),
                    )?;
                    insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        string_entity,
                        ControlComponent,
                    )?;
                    insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        string_entity,
                        PositionComponent::default(),
                    )?;
                    insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        string_entity,
                        GlobalPositionComponent::default(),
                    )?;
                    insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        string_entity,
                        ParentEntityComponent::new(list_control_entity),
                    )?;
                    insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        string_entity,
                        StringComponent::default(),
                    )?;
                    insert_entity_component(
                        &mut db.component_storage,
                        &mut db.entity_component_directory,
                        string_entity,
                        PancursesColorPairComponent::new(PancursesColorPair::default()),
                    )?;

                    if db.entity_component_directory.entity_has_component::<DebugExcludeComponent>(&list_control_entity) {
                        insert_entity_component(
                            &mut db.component_storage,
                            &mut db.entity_component_directory,
                            string_entity,
                            DebugExcludeComponent,
                        )?;
                    }

                    string_entities.push(string_entity);
                }

                // Create or update string components for this set of strings
                while string_entities.len() > string_count {
                    if let Some(string_entity) = string_entities.pop() {
                        db.destroy_entity(string_entity)?;
                    }
                }

                let local_mouse_position =
                    match get_entity_component::<CS, CD, LocalMousePositionComponent>(
                        &db.component_storage,
                        &db.entity_component_directory,
                        list_control_entity,
                    ) {
                        Ok(local_mouse_position_component) => {
                            Some(local_mouse_position_component.get_local_mouse_position())
                        }
                        Err(_) => None,
                    };

                let mut y = 0i64;
                for (string_index, strings) in string_list.iter().enumerate() {
                    let mut done = false;
                    for string in strings {
                        let string_entity = string_entities[y as usize];

                        // Update each string entity's position
                        get_entity_component_mut::<CS, CD, PositionComponent>(
                            &mut db.component_storage,
                            &mut db.entity_component_directory,
                            string_entity,
                        )?
                        .set_position(IVector2(0, y));

                        // Update each string entity's text
                        get_entity_component_mut::<CS, CD, StringComponent>(
                            &mut db.component_storage,
                            &mut db.entity_component_directory,
                            string_entity,
                        )?
                        .set_data(string.clone());

                        // Update color pair based on focused item
                        let focused_item = match list_index_entity {
                            Some(list_index_entity) => {
                                match get_entity_component_mut::<CS, CD, IntRangeComponent>(
                                    &mut db.component_storage,
                                    &mut db.entity_component_directory,
                                    list_index_entity,
                                ) {
                                    Ok(int_range_component) => {
                                        let len = string_list.len();
                                        int_range_component.set_range(0..(len as i64));
                                        if let Some(IVector2(mouse_x, mouse_y)) =
                                            local_mouse_position
                                        {
                                            let range_x = 0i64..width;
                                            if range_x.contains(&mouse_x) && mouse_y == y {
                                                int_range_component.set_index(string_index as i64);
                                            }
                                        }
                                        Some(int_range_component.get_index())
                                    }
                                    Err(_) => None,
                                }
                            }
                            None => None,
                        };

                        let data = if Some(string_index as i64) == focused_item {
                            PancursesColorPair::new(
                                PancursesColor::new(0, 0, 0),
                                PancursesColor::new(1000, 1000, 1000),
                            )
                        } else {
                            PancursesColorPair::default()
                        };

                        get_entity_component_mut::<CS, CD, PancursesColorPairComponent>(
                            &mut db.component_storage,
                            &mut db.entity_component_directory,
                            string_entity,
                        )?
                        .set_data(data);

                        y += 1;
                        if y >= height {
                            done = true;
                            break;
                        }
                    }

                    if done {
                        break;
                    }
                }
            } else if self
                .list_string_entities
                .get(&list_control_entity)
                .is_some()
            {
                println!(
                    "Clearing string entities for list control entity {:?}",
                    &list_control_entity
                );
                // The list control's string list entity has been cleared, remove it from the set of string entities
                self.list_string_entities.remove(&list_control_entity);
            }
        }

        Ok(())
    }
}
