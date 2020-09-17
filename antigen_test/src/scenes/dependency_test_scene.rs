use antigen::{
    components::{
        CharComponent, ParentEntityComponent, PositionComponent, SizeComponent, VelocityComponent,
        WindowComponent,
    },
    entity_component_system::create_entity,
    entity_component_system::insert_entity_component,
    entity_component_system::Scene,
    entity_component_system::{
        entity_component_database::{ComponentStorage, EntityComponentDirectory},
        system_storage::SystemStorage,
        Assemblage, EntityComponentSystem, SystemRunner,
    },
    primitive_types::IVector2,
    systems::PositionIntegratorSystem,
};

use crate::components::{
    control_component::ControlComponent,
    pancurses_color_pair_component::PancursesColorPairComponent,
    pancurses_input_buffer_component::PancursesInputBufferComponent,
    pancurses_window_component::PancursesWindowComponent,
};
use crate::pancurses_color::{PancursesColor, PancursesColorPair};
use crate::systems::{
    InputVelocitySystem, PancursesInputSystem, PancursesRendererSystem, PancursesWindowSystem,
};

pub struct DependencyTestScene;

impl Scene for DependencyTestScene {
    fn register_systems<CS, CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CS, CD> + 'static,
        SR: SystemRunner + 'static,
    {
        // Resolution Strategy
        // Treat predicates as ref fallbacks for components that don't get read or written (ex. WindowComponent)
        // For each mutable component reference in a given system
        //   If no other systems take a mutable reference to the same component, this system is the component's entrypoint
        //   If any other system takes a mutable reference to the same component, the relation should be ignored and dependencies inferred from other components
        //   Otherwise, non-mutable references to the same component should be checked and stored as a System > System map

        // pred: (WindowComponent, PancursesWindowComponent, SizeComponent)
        // ref: PancursesWindowComponent, SizeComponent, CharComponent, PancursesColorPairComponent, StringComponent
        // mut: SizeComponent, PancursesColorSetComponent, PancursesWindowComponent
        let pancurses_window_system =
            PancursesWindowSystem::new(&mut ecs.entity_component_database);
        ecs.push_system("Pancurses Window", pancurses_window_system);

        // pred: (WindowComponent, PancursesWindowComponent)
        // ref: PancursesWindowComponent
        // mut: PancursesMouseComponent, PancursesInputBufferComponent
        ecs.push_system("Pancurses Input", PancursesInputSystem::new(1));

        // pred: (PancursesInputBufferComponent, VelocityComponent)
        // ref: PancursesInputBufferComponent
        // mut: VelocityComponent
        ecs.push_system("Input Velocity", InputVelocitySystem::new());

        // pred: (PositionComponent, VelocityComponent)
        // ref: VelocityComponent
        // mut: PositionComponent
        ecs.push_system("Position Integrator", PositionIntegratorSystem::new());

        // pred: PancursesColorSetComponent, (ControlComponent, ParentEntityComponent, PositionComponent), (WindowComponent, PancursesWindowComponent, SizeComponent)
        // ref: ParentEntityComponent, ZIndexComponent, ChildEntitiesComponent, PancursesWindowComponent, ParentEntityComponent, PancursesWindowComponent,
        //      ParentEntityComponent, GlobalPositionComponent, PositionComponent, PancursesColorPairComponent, CharComponent, SizeComponent, StringComponent, PancursesWindowComponent
        // mut: PancursesColorSetComponent
        ecs.push_system("Pancurses Renderer", PancursesRendererSystem::new());

        Ok(())
    }

    fn create_entities<CS, CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CS, CD, SS, SR>,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
        SS: SystemStorage<CS, CD>,
        SR: SystemRunner,
    {
        // Create Main Window
        let main_window_entity = create_entity(
            &mut ecs.entity_component_database.component_storage,
            &mut ecs.entity_component_database.entity_component_directory,
            &mut ecs.entity_component_database.callback_manager,
            Some("Main Window"),
        )?;
        {
            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                main_window_entity,
                WindowComponent,
            )?;
            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                main_window_entity,
                PancursesWindowComponent::default(),
            )?;
            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                main_window_entity,
                SizeComponent::new(IVector2(64, 32)),
            )?;
        }

        // Create Player
        let mut player_assemblage = Assemblage::build(
            "Player Entity",
            "Controllable ASCII character with position and velocity",
        )
        .add_component(ControlComponent)?
        .add_component(PositionComponent::new(IVector2(1, 1)))?
        .add_component(VelocityComponent::new(IVector2(1, 1)))?
        .add_component(CharComponent::new('@'))?
        .add_component(PancursesColorPairComponent::new(PancursesColorPair::new(
            PancursesColor::new(1000, 600, 1000),
            PancursesColor::new(1000, 1000, 1000),
        )))?
        .add_component(PancursesInputBufferComponent::default())?
        .finish();

        let test_player_entity = player_assemblage
            .create_and_assemble_entity(&mut ecs.entity_component_database, Some("Test Player"))?;
        {
            insert_entity_component(
                &mut ecs.entity_component_database.component_storage,
                &mut ecs.entity_component_database.entity_component_directory,
                test_player_entity,
                ParentEntityComponent::new(main_window_entity),
            )?;
        }

        Ok(())
    }
}
