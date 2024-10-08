use bevy::{ecs::system::EntityCommands, prelude::*, utils::hashbrown::HashMap};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
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

#[derive(Default)]
pub struct AssetPlugin {
    registry: ComponentRegistry,
}

impl AssetPlugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<C>(mut self, name: &str) -> Self
    where
        C: Component + Clone + DeserializeOwned,
    {
        self.registry.insert::<C>(name);
        self
    }
}

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.registry.clone())
            .init_resource::<BundleRegistry>();
    }
}

#[derive(Clone, Default, Resource)]
struct ComponentRegistry {
    components: HashMap<
        String,
        Arc<dyn Fn(Value) -> Box<dyn Fn(EntityCommands) + Send + Sync> + Send + Sync>,
    >,
}

impl ComponentRegistry {
    pub fn insert<C>(&mut self, name: &str)
    where
        C: Component + Clone + DeserializeOwned,
    {
        self.components.insert(
            name.to_string(),
            Arc::new(|value| {
                let x = serde_json::from_value::<C>(value).unwrap();
                Box::new(move |mut commands| {
                    commands.insert(x.clone());
                })
            }),
        );
    }
}

#[derive(Clone, Default, Resource)]
pub struct BundleRegistry {
    bundles: HashMap<String, Bundler>,
}

impl BundleRegistry {
    pub fn spawn(&self, name: &str, entity_commands: EntityCommands) {
        self.bundles.get(name).unwrap().spawn(entity_commands);
    }
}
