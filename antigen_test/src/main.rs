mod components;
mod pancurses_color;
mod scenes;
mod systems;

use std::time::Duration;

use antigen::{
    entity_component_system::Scene,
    entity_component_system::{
        entity_component_database::{HeapComponentStorage, SingleThreadedDirectory},
        system_runner::SingleThreadedSystemRunner,
        system_storage::HeapSystemStorage,
        EntityComponentSystem, SystemError,
    },
    profiler::Profiler,
};

fn main() {
    if let Err(err) = main_internal() {
        match err {
            SystemError::Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1)
            }
            SystemError::Quit => std::process::exit(0),
        }
    }
}

fn main_internal() -> Result<(), SystemError> {
    let mut ecs = EntityComponentSystem::<
        HeapComponentStorage,
        SingleThreadedDirectory,
        HeapSystemStorage<HeapComponentStorage, SingleThreadedDirectory>,
        SingleThreadedSystemRunner,
    >::default();

    scenes::AntigenDebugScene::load(&mut ecs)?;

    // Main loop
    let frame_time_target = Duration::from_secs_f32(1.0 / 60.0);
    loop {
        let main_loop_profiler = Profiler::start("Main Loop");
        ecs.run()?;
        let delta = main_loop_profiler.finish();

        // Sleep if framerate target is exceeded - prevents deadlock when pancurses stops being able to poll input after window close
        if delta < frame_time_target {
            let delta = frame_time_target - delta;
            std::thread::sleep(delta);
        }
    }
}
