use bevy::{ecs::system::EntityCommands, prelude::*};
use std::sync::Arc;

pub struct Bundler {
    spawn_fn: Arc<dyn Fn(EntityCommands) + Send + Sync>,
}

impl Default for Bundler {
    fn default() -> Self {
        Self::new()
    }
}

impl Bundler {
    pub fn new() -> Self {
        Self {
            spawn_fn: Arc::new(|_| {}),
        }
    }

    pub fn insert<B>(&mut self, bundle: B)
    where
        B: Bundle + Clone,
    {
        let f = self.spawn_fn.clone();
        self.spawn_fn = Arc::new(move |mut entity_commands| {
            entity_commands.insert(bundle.clone());
            (f)(entity_commands);
        });
    }

    pub fn spawn(&self, entity_commands: EntityCommands) {
        (self.spawn_fn)(entity_commands)
    }
}
