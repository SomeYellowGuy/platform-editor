use std::collections::HashMap;

/// A map storing each component (via an ID) and giving each one a priority value to be rendered.
///
/// A higher priority means appearing later in the `iter` and `iter_mut` methods.
pub struct ComponentMap<C> {
    components: HashMap<String, C>,
    render_priorities: HashMap<String, i32>,
    logic_priorities: HashMap<String, i32>
}

#[derive(Debug, Copy, Clone)]
pub enum ComponentMapQueryType {
    Render,
    Logic
}

macro_rules! priorities_for_mut {
    ($target:expr, $ty:expr) => {
        match $ty {
            ComponentMapQueryType::Render => &mut $target.render_priorities,
            ComponentMapQueryType::Logic => &mut $target.logic_priorities,
        }
    };
}

impl<C> ComponentMap<C> {
    /// Creates a new map.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            render_priorities: HashMap::new(),
            logic_priorities: HashMap::new(),
        }
    }

    /// Inserts a new component in this map.
    pub fn insert(&mut self, id: &str, component: C, render_priority: i32, logic_priority: i32) {
        self.components.insert(id.to_string(), component);
        self.render_priorities.insert(id.to_string(), render_priority);
        self.logic_priorities.insert(id.to_string(), logic_priority);
    }

    /// Removes a component, with the provided key, from this map and returns the component
    /// if one could be removed.
    pub fn remove(&mut self, id: &str) -> Option<C> {
        self.render_priorities.remove(id);
        self.logic_priorities.remove(id);
        self.components.remove(id)
    }

    fn priorities(&self, ty: ComponentMapQueryType) -> &HashMap<String, i32> {
        match ty {
            ComponentMapQueryType::Render => &self.render_priorities,
            ComponentMapQueryType::Logic => &self.logic_priorities,
        }
    }

    /// Provides an [`Iterator`] with the provided priority type.
    /// 
    /// This iterator starts from the lowest-prioritized component, where
    /// each item is a reference to a component.
    pub fn ascending_iter(&self, ty: ComponentMapQueryType) -> impl Iterator<Item = (&String, &C)> {
        let mut items: Vec<_> = self.components.iter().collect();
        items.sort_by_key(|(s, _)| *self.priorities(ty).get(*s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    /// 
    /// This iterator starts from the highest-prioritized component, where
    /// each item is a reference to a component.
    pub fn descending_iter(&self, ty: ComponentMapQueryType) -> impl Iterator<Item = (&String, &C)> {
        let mut items: Vec<_> = self.components.iter().collect();
        items.sort_by_key(|(s, _)| -*self.priorities(ty).get(*s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    /// 
    /// This iterator starts from the lowest-prioritized component, where
    /// each item is a mutable reference to a component.
    pub fn ascending_iter_mut(&mut self, ty: ComponentMapQueryType) -> impl Iterator<Item = (&String, &mut C)> {
        let mut items: Vec<_> = self.components.iter_mut().collect();
        let priorities = priorities_for_mut!(self, ty);
        items.sort_by_key(|(s, _)| -*priorities.get(*s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    /// 
    /// This iterator starts from the highest-prioritized component, where
    /// each item is a mutable reference to a component.
    pub fn descending_iter_mut(&mut self, ty: ComponentMapQueryType) -> impl Iterator<Item = (&String, &mut C)> {
        let mut items: Vec<_> = self.components.iter_mut().collect();
        let priorities = priorities_for_mut!(self, ty);
        items.sort_by_key(|(s, _)| -*priorities.get(*s).unwrap());
        items.into_iter()
    }
}

impl<C> Default for ComponentMap<C> {
    fn default() -> Self {
        Self::new()
    }
}
