use std::fmt::Debug;

use crate::{
    entity_component_system::ComponentDebugTrait, entity_component_system::ComponentTrait,
};

#[derive(Clone, PartialEq, PartialOrd)]
pub struct SoftwareFramebuffer<T>
where
    T: Copy + Clone,
{
    clear_data: T,
    color_buffer: Vec<T>,
    z_buffer: Vec<Option<i64>>,
}

impl<T> Debug for SoftwareFramebuffer<T>
where
    T: Copy + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPUFramebufferComponent").finish()
    }
}

impl<T> SoftwareFramebuffer<T>
where
    T: Copy + Clone,
{
    pub fn new(clear_data: T) -> SoftwareFramebuffer<T> {
        SoftwareFramebuffer {
            clear_data,
            color_buffer: Vec::new(),
            z_buffer: Vec::new(),
        }
    }

    pub fn get_color_buffer(&self) -> Vec<T> {
        self.color_buffer.clone()
    }

    pub fn get_z_buffer(&self) -> Vec<Option<i64>> {
        self.z_buffer.clone()
    }

    pub fn clear(&mut self) {
        let clear_data = self.clear_data;

        self.color_buffer
            .iter_mut()
            .for_each(|color| *color = clear_data);

        self.z_buffer.iter_mut().for_each(|z| *z = None);
    }

    pub fn draw(&mut self, x: i64, y: i64, window_width: i64, data: T, z: i64) {
        let idx = y * window_width + x;
        let idx = idx as usize;

        let existing_z = self.z_buffer[idx];

        if let Some(existing_z) = existing_z {
            if existing_z > z {
                return;
            }
        }

        self.color_buffer[idx] = data;
        self.z_buffer[idx] = Some(z);
    }

    pub fn resize(&mut self, new_size: usize) {
        if self.color_buffer.len() != new_size {
            self.color_buffer.resize(new_size, self.clear_data);
        }

        if self.z_buffer.len() != new_size {
            self.z_buffer.resize(new_size, None);
        }
    }
}

impl<T> ComponentTrait for SoftwareFramebuffer<T> where T: Copy + Clone + 'static {}

impl<T> ComponentDebugTrait for SoftwareFramebuffer<T>
where
    T: Copy + Clone,
{
    fn get_name() -> String {
        format!("Software Framebuffer ({})", std::any::type_name::<T>())
    }

    fn get_description() -> String {
        format!(
            "Framebuffer holding data of type {}",
            std::any::type_name::<T>()
        )
    }
}