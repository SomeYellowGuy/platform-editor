use std::collections::HashMap;

/// A map storing each component (via an ID) and giving each one a priority value to be rendered.
///
/// A higher priority means appearing later in the `iter` and `iter_mut` methods.
pub struct ComponentMap<C> {
    components: HashMap<String, C>,
    render_priorities: HashMap<String, i32>,
}

impl<C> ComponentMap<C> {
    /// Creates a new map.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            render_priorities: HashMap::new(),
        }
    }

    /// Inserts a new component in this map.
    pub fn insert(&mut self, id: &str, component: C, priority: i32) {
        self.components.insert(id.to_string(), component);
        self.render_priorities.insert(id.to_string(), priority);
    }

    /// Removes a component, with the provided key, from this map and returns the component
    /// if one could be removed.
    pub fn remove(&mut self, id: &str) -> Option<C> {
        self.render_priorities.remove(id);
        self.components.remove(id)
    }

    /// Provides an [`Iterator`], starting from the lowest-prioritized component, where
    /// each item is a reference to a component.
    ///
    /// # Notes
    ///
    /// This is useful for rendering.
    pub fn ascending_iter(&self) -> impl Iterator<Item = (&String, &C)> {
        let mut items: Vec<_> = self.components.iter().collect();
        items.sort_by_key(|(s, _)| *self.render_priorities.get(*s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`], starting from the highest-prioritized component, where
    /// each item is a mutable reference to a component.
    ///
    /// # Notes
    ///
    /// This is useful for executing logic.
    pub fn descending_iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut C)> {
        let mut items: Vec<_> = self.components.iter_mut().collect();
        items.sort_by_key(|(s, _)| -*self.render_priorities.get(*s).unwrap());
        items.into_iter()
    }
}

impl<C> Default for ComponentMap<C> {
    fn default() -> Self {
        Self::new()
    }
}