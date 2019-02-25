use specs::prelude::*;
use crate::components::{register_components, Name};

pub struct TestHelper {
    pub world: World,

    entities: Option<Vec<Entity>>,
}

impl TestHelper {
    // Builds a worlds and registers all components.
    pub fn new() -> Self {
        let mut world = World::new();
        register_components(&mut world);

        Self { world, entities: None }
    }

    // Synchronously runs an individual system.
    pub fn run<'a, S: RunNow<'a>>(&'a self, mut system: S) {
        system.run_now(&self.world.res);
    }

    // Get an entity by name. Panics if no entity exists.
    pub fn entity(&mut self, s: &str) -> Entity {
        self.try_entity(s).expect(&format!("No entity called {}", s))
    }

    // Returns whether an entity with a given name exists.
    pub fn exists(&mut self, s: &str) -> bool {
        self.try_entity(s).is_some()
    }

    // Tries to get an entity by name, or returns None.
    pub fn try_entity(&mut self, s: &str) -> Option<Entity> {
        self.world.maintain();

        let names = self.world.read_storage::<Name>();
        let entities = self.world.entities();

        (&names, &entities).join().find(|(n, _)| n.string == s).map(|(_, e)| e)
    }

    // Return a Vec of all entities and caches them.
    pub fn entities(&mut self) -> &Vec<Entity> {
        if self.entities.is_none() {
            self.world.maintain();
            self.entities = Some(self.world.entities().join().collect());
        }

        self.entities.as_ref().unwrap()
    }

    // Clears the entities cache. Call after adding/removing entities.
    pub fn refresh(&mut self) {
        self.entities = None;
    }

    // Read a component for an entity. Infers the type of component.
    pub fn read<C: Component + Clone>(&mut self, entity: Entity) -> Option<C> {
        self.world.read_storage::<C>().get(entity).cloned()
    }

    // Returns whether an entity has a given component.
    pub fn has<C: Component>(&mut self, entity: Entity) -> bool {
        self.world.read_storage::<C>().get(entity).is_some()
    }
}
