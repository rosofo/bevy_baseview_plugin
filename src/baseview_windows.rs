use std::default;

use bevy::{prelude::*, utils::HashMap};
use bevy::window::Window;
use raw_window_handle::HasRawWindowHandle;


use crate::{BaseviewWindow, BaseviewWindowInfo};

#[derive(Debug, Default)]
pub struct BaseviewWindows {
    pub window_entity_to_baseview: HashMap<Entity, u64>,
    pub baseview_to_window_entity: HashMap<u64, Entity>,
    // Some window functions, such as `set_window_icon` can only be used from the main thread. If
    // they are used in another thread, the app will hang. This marker ensures `WinitWindows` is
    // only ever accessed with bevy's non-send functions and in NonSend systems.
    _not_send_sync: core::marker::PhantomData<*const ()>,
}

impl BaseviewWindows {
    pub(crate) fn create_window(
        &mut self,
        window_id: Entity,
        window_descriptor: &Window,
        baseview_window_info: BaseviewWindowInfo,
    ) -> Window {
        let BaseviewWindowInfo {
            window_open_options,
            parent_win,
        } = baseview_window_info;

        let baseview_window = BaseviewWindow::new();
        let baseview_window_id = baseview_window.id();

        let scale_factor = match window_open_options.scale {
            baseview::WindowScalePolicy::SystemScaleFactor => 1.0,
            baseview::WindowScalePolicy::ScaleFactor(scale) => scale,
        };

        let window_info =
            baseview::WindowInfo::from_logical_size(window_open_options.size, scale_factor);
        // TODO: Should probably add utilities to baseview to expose easier
        // scaling.
        let phy_size = window_open_options.size.to_physical(&window_info);

        baseview::Window::open_parented(&parent_win, window_open_options, |_| baseview_window);
        self.baseview_to_window_entity
            .insert(baseview_window_id, window_id);
        self.window_entity_to_baseview
            .insert(window_id, baseview_window_id);

        Window {
            position,
            
            parent_win.raw_window_handle()
            ..default()
        }
    }

    pub fn get_window_id(&self, id: u64) -> Option<WindowId> {
        self.baseview_to_window_entity.get(&id).cloned()
    }
}
