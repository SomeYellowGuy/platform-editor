use std::{cmp::Reverse, collections::HashMap};

use crate::component::button::ButtonType;

pub mod button;
pub mod title;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ComponentId {
    Title,
    Button(ButtonType),
}

/// A map storing each component (via an ID) and giving each one a priority value to be rendered.
///
/// A higher priority means appearing later in the `iter` and `iter_mut` methods.
pub struct ComponentMap<C> {
    components: HashMap<ComponentId, C>,
    render_priorities: HashMap<ComponentId, i32>,
    logic_priorities: HashMap<ComponentId, i32>,
}

#[derive(Debug, Copy, Clone)]
pub enum ComponentMapQueryType {
    Render,
    Logic,
}

macro_rules! priorities_for_mut {
    ($target:expr, $ty:expr) => {
        match $ty {
            ComponentMapQueryType::Render => &$target.render_priorities,
            ComponentMapQueryType::Logic => &$target.logic_priorities,
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
    pub fn insert(
        &mut self,
        id: ComponentId,
        component: C,
        render_priority: i32,
        logic_priority: i32,
    ) {
        self.components.insert(id, component);
        self.render_priorities.insert(id, render_priority);
        self.logic_priorities.insert(id, logic_priority);
    }

    /// Removes a component, with the provided key, from this map (if any).
    pub fn remove(&mut self, id: ComponentId) {
        self.render_priorities.remove(&id);
        self.logic_priorities.remove(&id);
        self.components.remove(&id);
    }

    /// Removes components whose key satisfies the given predicate, from this map (if any).
    pub fn remove_all(&mut self, predicate: impl Fn(ComponentId) -> bool) {
        let ids: Vec<_> = self
            .components
            .keys()
            .filter_map(|k| predicate(*k).then_some(*k))
            .collect();
        for component in ids {
            self.remove(component);
        }
    }

    fn priorities(&self, ty: ComponentMapQueryType) -> &HashMap<ComponentId, i32> {
        match ty {
            ComponentMapQueryType::Render => &self.render_priorities,
            ComponentMapQueryType::Logic => &self.logic_priorities,
        }
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the lowest-prioritized component, where
    /// each item is a reference to a component.
    pub fn ascending_iter(
        &self,
        ty: ComponentMapQueryType,
    ) -> impl Iterator<Item = (ComponentId, &C)> {
        let mut items: Vec<_> = self.components.iter().map(|(k, v)| (*k, v)).collect();
        let priorities = self.priorities(ty);
        items.sort_unstable_by_key(|(s, _)| *priorities.get(s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the highest-prioritized component, where
    /// each item is a reference to a component.
    pub fn descending_iter(
        &self,
        ty: ComponentMapQueryType,
    ) -> impl Iterator<Item = (ComponentId, &C)> {
        let mut items: Vec<_> = self.components.iter().map(|(k, v)| (*k, v)).collect();
        let priorities = self.priorities(ty);
        items.sort_unstable_by_key(|(s, _)| Reverse(*priorities.get(s).unwrap()));
        items.into_iter()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the lowest-prioritized component, where
    /// each item is a mutable reference to a component.
    pub fn ascending_iter_mut(
        &mut self,
        ty: ComponentMapQueryType,
    ) -> impl Iterator<Item = (ComponentId, &mut C)> {
        let mut items: Vec<_> = self.components.iter_mut().map(|(k, v)| (*k, v)).collect();
        let priorities = priorities_for_mut!(self, ty);
        items.sort_unstable_by_key(|(s, _)| *priorities.get(s).unwrap());
        items.into_iter()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the highest-prioritized component, where
    /// each item is a mutable reference to a component.
    pub fn descending_iter_mut(
        &mut self,
        ty: ComponentMapQueryType,
    ) -> impl Iterator<Item = (ComponentId, &mut C)> {
        let mut items: Vec<_> = self.components.iter_mut().map(|(k, v)| (*k, v)).collect();
        let priorities = priorities_for_mut!(self, ty);
        items.sort_unstable_by_key(|(s, _)| Reverse(*priorities.get(s).unwrap()));
        items.into_iter()
    }
}

impl<C> Default for ComponentMap<C> {
    fn default() -> Self {
        Self::new()
    }
}
