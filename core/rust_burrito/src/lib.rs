mod component;
mod entity;
mod event;
mod params;
mod plugin;
mod resource;
mod system;
//  mod wish_impl;

#[cfg(test)]
mod test;

pub use component::*;
pub use entity::*;
pub use event::*;
pub use params::*;
pub use plugin::*;
pub use resource::*;
pub use system::*;

pub use mewo_ecs::{Entity, Executor, Galaxy, StraightExecutor};

use mewo_ecs::RawPlugin;

pub struct RustRuntime {
    plugins: Vec<RawPlugin>,
}

impl RustRuntime {
    pub fn create() -> Self {
        RustRuntime {
            plugins: Vec::new(),
        }
    }

    pub fn raw_plugin(mut self, plugin: RawPlugin) -> Self {
        self.plugins.push(plugin);
        self
    }

    pub fn plugin<P: Plugin>(mut self) -> Self {
        let pb = PluginBuilder::create::<P>();
        self.plugins.push(P::plugin(pb).build());
        self
    }

    pub fn done(self) -> Vec<RawPlugin> {
        self.plugins
    }
}
