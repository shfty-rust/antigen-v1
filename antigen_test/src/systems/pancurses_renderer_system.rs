use std::collections::HashMap;

use crate::{
    components::fill_component::FillComponent,
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::{
        control_component::ControlComponent, pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::PancursesColor,
    pancurses_color::PancursesColorPair,
};
use antigen::{
    components::ColorComponent,
    components::{
        CharComponent, ChildEntitiesComponent, GlobalPositionComponent, ParentEntityComponent,
        PositionComponent, SizeComponent, StringComponent, WindowComponent, ZIndexComponent,
    },
    entity_component_system::SystemDebugTrait,
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, EntityComponentDirectory, EntityID,
        SystemError, SystemTrait,
    },
    primitive_types::ColorRGB,
    primitive_types::Vector2I,
};
use pancurses::{ToChtype, Window};

#[derive(Debug)]
pub struct PancursesRendererSystem;

impl PancursesRendererSystem {
    pub fn new() -> PancursesRendererSystem {
        PancursesRendererSystem
    }

    fn render_string(
        &self,
        window: &Window,
        window_size: Vector2I,
        position: Vector2I,
        string: &str,
        color_pair: i16,
    ) {
        let Vector2I(window_width, window_height) = window_size;
        let Vector2I(x, y) = position;

        let len = string.len() as i64;

        let mut new_x = x;
        let mut new_str = string.to_string();
        if x < -len {
            new_str.clear();
        } else if x < 0 {
            new_x = 0;
            new_str = string[(len - (len + x)) as usize..].into();
        }

        if new_x > window_width {
            new_str.clear();
        } else if new_x > window_width - new_str.len() as i64 {
            new_str = new_str[..(window_width - new_x) as usize].into();
        }

        let len = new_str.len() as i64;
        if len <= 0 || y < 0 || y >= window_height {
            return;
        }

        let mut y = y as i32;
        window.mv(y, x as i32);
        for char in new_str.chars() {
            if char == '\n' {
                y += 1;
                window.mv(y, x as i32);
            } else {
                window.addch(char.to_chtype() | pancurses::COLOR_PAIR(color_pair as u64));
            }
        }
    }

    fn render_rect(
        &self,
        window: &Window,
        window_size: Vector2I,
        position: Vector2I,
        size: Vector2I,
        char: char,
        color_pair: i16,
        filled: bool,
    ) {
        let Vector2I(window_width, window_height) = window_size;
        let Vector2I(pos_x, pos_y) = position;
        let Vector2I(width, height) = size;

        let mut w = width;
        let width_delta = (pos_x + w) - window_width;
        if width_delta > 0 {
            w -= width_delta;
        }

        let mut h = height;
        let height_delta = (pos_y + h) - window_height;
        if height_delta > 0 {
            h -= height_delta;
        }

        let mut x = pos_x;
        if x < 0 {
            w += x;
            x = 0;
        }

        let mut y = pos_y;
        if y < 0 {
            h += y;
            y = 0;
        }

        if w == 0 || h == 0 {
            return;
        }

        let char = char.to_chtype();
        if filled {
            if w >= h {
                for y in y..y + h {
                    window.mv(y as i32, x as i32);
                    window.hline(char | pancurses::COLOR_PAIR(color_pair as u64), w as i32);
                }
            } else {
                for x in x..x + w {
                    window.mv(y as i32, x as i32);
                    window.vline(char | pancurses::COLOR_PAIR(color_pair as u64), h as i32);
                }
            }
        } else {
            window.mv(y as i32, x as i32);
            window.hline(char | pancurses::COLOR_PAIR(color_pair as u64), w as i32);

            window.mv((y + h - 1) as i32, x as i32);
            window.hline(char | pancurses::COLOR_PAIR(color_pair as u64), w as i32);

            window.mv((y + 1) as i32, x as i32);
            window.vline(
                char | pancurses::COLOR_PAIR(color_pair as u64),
                (h - 2) as i32,
            );

            window.mv((y + 1) as i32, (x + w - 1) as i32);
            window.vline(
                char | pancurses::COLOR_PAIR(color_pair as u64),
                (h - 2) as i32,
            );
        }
    }
}

impl<CS, CD> SystemTrait<CS, CD> for PancursesRendererSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Fetch color set entity
        let color_set_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<PancursesColorSetComponent>(entity_id)
            });
        let color_set_entity = color_set_entity.expect("Color set entity does not exist");

        // Fetch control entities
        let control_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<ControlComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<ParentEntityComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<PositionComponent>(entity_id)
                });

        // Filter out non-root controls and sort
        let mut root_controls: Vec<EntityID> = control_entities
            .iter()
            .filter(|entity_id| {
                let parent_entity_component =
                    match db.get_entity_component::<ParentEntityComponent>(**entity_id) {
                        Ok(parent_entity_component) => parent_entity_component,
                        Err(_) => return false,
                    };

                let parent_id = parent_entity_component.get_parent_id();
                !db.entity_component_directory
                    .entity_has_component::<ControlComponent>(&parent_id)
            })
            .copied()
            .collect();

        root_controls.sort();

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let mut z_layers: HashMap<i64, Vec<EntityID>> = HashMap::new();

        fn populate_z_layers<CS, CD>(
            db: &SystemInterface<CS, CD>,
            entity_id: EntityID,
            z_layers: &mut HashMap<i64, Vec<EntityID>>,
            mut z_index: i64,
        ) -> Result<(), String>
        where
            CS: ComponentStorage,
            CD: EntityComponentDirectory,
        {
            let z_layer = match z_layers.get_mut(&z_index) {
                Some(z_layer) => z_layer,
                None => {
                    z_layers.insert(z_index, Vec::new());
                    match z_layers.get_mut(&z_index) {
                        Some(z_layer) => z_layer,
                        None => return Err(format!("Failed to get Z layer {}", z_index)),
                    }
                }
            };

            if db
                .get_entity_component::<ControlComponent>(entity_id)
                .is_ok()
            {
                z_index = match db.get_entity_component::<ZIndexComponent>(entity_id) {
                    Ok(z_index_component) => z_index_component.get_z(),
                    Err(_) => z_index,
                };

                z_layer.push(entity_id);
            }

            if let Ok(child_entities_component) =
                db.get_entity_component::<ChildEntitiesComponent>(entity_id)
            {
                for child_id in child_entities_component.get_child_ids() {
                    populate_z_layers(db, *child_id, z_layers, z_index)?;
                }
            }

            Ok(())
        };

        for entity_id in root_controls {
            populate_z_layers(db, entity_id, &mut z_layers, 0)?;
        }

        let mut control_entities: Vec<EntityID> = Vec::new();
        let mut layer_keys: Vec<i64> = z_layers.keys().copied().collect();
        layer_keys.sort();
        for i in layer_keys {
            let z_layer = match z_layers.get_mut(&i) {
                Some(z_layer) => z_layer,
                None => return Err(format!("Failed to get Z layer {}", i).into()),
            };
            control_entities.append(z_layer);
        }

        // Fetch window entities
        let window_entities =
            db.entity_component_directory
                .get_entities_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<WindowComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<PancursesWindowComponent>(entity_id)
                        && db
                            .entity_component_directory
                            .entity_has_component::<SizeComponent>(entity_id)
                });

        // Erase existing framebuffer
        for entity_id in &window_entities {
            db.get_entity_component::<PancursesWindowComponent>(*entity_id)?
                .get_window()
                .map(|window| window.erase());
        }

        // Render Entities
        for entity_id in control_entities {
            // Search up parent chain for window component
            let parent_entity_component =
                match db.get_entity_component::<ParentEntityComponent>(entity_id) {
                    Ok(parent_entity_component) => parent_entity_component,
                    Err(err) => return Err(err.into()),
                };

            let mut candidate_id = parent_entity_component.get_parent_id();
            let mut parent_id: Option<EntityID> = None;

            loop {
                if db
                    .get_entity_component::<PancursesWindowComponent>(candidate_id)
                    .is_ok()
                {
                    parent_id = Some(candidate_id);
                    break;
                }

                match db.get_entity_component::<ParentEntityComponent>(candidate_id) {
                    Ok(parent_entity_component) => {
                        candidate_id = parent_entity_component.get_parent_id()
                    }
                    Err(_) => break,
                }
            }

            // Skip rendering this entity if it has no window ancestor
            let parent_id = match parent_id {
                Some(parent_id) => parent_id,
                None => continue,
            };

            // Get Position
            let Vector2I(x, y) = if let Ok(global_position_component) =
                db.get_entity_component::<GlobalPositionComponent>(entity_id)
            {
                global_position_component.get_global_position()
            } else {
                match db.get_entity_component::<PositionComponent>(entity_id) {
                    Ok(position_component) => position_component.get_position(),
                    Err(err) => return Err(err.into()),
                }
            };

            // Get Color
            let color = match db.get_entity_component::<ColorComponent>(entity_id) {
                Ok(color_component) => *color_component.get_data(),
                Err(_) => ColorRGB(1.0, 1.0, 1.0),
            };

            let char = match db.get_entity_component::<CharComponent>(entity_id) {
                Ok(char_component) => *char_component.get_data(),
                Err(_) => ' ',
            };

            let string_color_pair_idx = db
                .get_entity_component_mut::<PancursesColorSetComponent>(color_set_entity)?
                .get_color_pair_idx(&PancursesColorPair::new(
                    color.into(),
                    PancursesColor::new(0, 0, 0),
                ));

            let rect_color_pair_idx = db
                .get_entity_component_mut::<PancursesColorSetComponent>(color_set_entity)?
                .get_color_pair_idx(&PancursesColorPair::new(
                    PancursesColor::new(0, 0, 0),
                    color.into(),
                ));

            let window_component =
                db.get_entity_component::<PancursesWindowComponent>(parent_id)?;

            if let Some(window) = window_component.get_window() {
                let (window_height, window_width) = window.get_max_yx();
                let (window_width, window_height) = (window_width as i64, window_height as i64);

                if db
                    .entity_component_directory
                    .entity_has_component::<SizeComponent>(&entity_id)
                {
                    let filled = db
                        .entity_component_directory
                        .entity_has_component::<FillComponent>(&entity_id);

                    let Vector2I(width, height) = db
                        .get_entity_component::<SizeComponent>(entity_id)?
                        .get_size();
                    self.render_rect(
                        window,
                        Vector2I(window_width, window_height),
                        Vector2I(x, y),
                        Vector2I(width, height),
                        char,
                        rect_color_pair_idx,
                        filled,
                    );
                } else if db
                    .entity_component_directory
                    .entity_has_component::<StringComponent>(&entity_id)
                    || db
                        .entity_component_directory
                        .entity_has_component::<CharComponent>(&entity_id)
                {
                    let string = if let Ok(string_component) =
                        db.get_entity_component::<StringComponent>(entity_id)
                    {
                        string_component.get_data().clone()
                    } else if let Ok(char_component) =
                        db.get_entity_component::<CharComponent>(entity_id)
                    {
                        char_component.get_data().to_string()
                    } else {
                        return Err("No valid string component".into());
                    };

                    for (i, string) in string.split('\n').enumerate() {
                        self.render_string(
                            window,
                            Vector2I(window_width, window_height),
                            Vector2I(x, y + i as i64),
                            string,
                            string_color_pair_idx,
                        )
                    }
                }
            }
        }

        for entity_id in &window_entities {
            db.get_entity_component::<PancursesWindowComponent>(*entity_id)?
                .get_window()
                .map(|window| window.refresh());
        }

        Ok(())
    }
}

impl SystemDebugTrait for PancursesRendererSystem {
    fn get_name() -> &'static str {
        "Pancurses Renderer"
    }
}
