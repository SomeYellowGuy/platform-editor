use std::{
    collections::{
        HashMap,
        hash_map::{Iter, IterMut},
    },
    time::Instant,
};

use crate::component::{level::end_dialog::EndDialogButtonType, title::button::ButtonType};

pub mod level;
pub mod level_select;
pub mod title;

/// Represents the back button of a screen.
pub struct BackButtonBase {
    pub hold_time: u32,
    pub pos: (i32, i32),
    pub mode: BackButtonMode,
}

/// Specifies the behavior of the back button when clicked.
#[derive(Debug, Clone, Copy)]
pub enum BackButtonMode {
    BackToTitle,
}

impl BackButtonBase {
    pub fn new(pos: (i32, i32), mode: BackButtonMode) -> Self {
        Self {
            pos,
            hold_time: 0,
            mode,
        }
    }
}

impl Hold for BackButtonBase {
    const MAX_HOLD_TIME: u32 = 400_000_000;

    fn hold_time(&self) -> u32 {
        self.hold_time
    }

    fn set_hold_time(&mut self, new_time: u32) {
        self.hold_time = new_time;
    }
}

pub const NO_LOGIC_PRIORITY: i32 = i32::MIN;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ComponentId {
    Title,
    Button(ButtonType),
    LevelSelectButton(usize),
    LevelSelectHeader,
    BackButton,
    Board,
    ItemTab,
    BottomBar,
    EndDialog,
    EndDialogButton(EndDialogButtonType),

    Other(usize),
}

/// A map storing each component (via an ID) and giving each one a priority value to be rendered.
///
/// A higher priority means appearing later in the `iter` and `iter_mut` methods.
pub struct ComponentMap<C> {
    components: HashMap<ComponentId, C>,
    render_priorities: HashMap<ComponentId, i32>,
    logic_priorities: HashMap<ComponentId, i32>,

    cached_render_ids: Vec<ComponentId>,
    cached_logic_ids: Vec<ComponentId>,
}

#[derive(Debug, Copy, Clone)]
pub enum ComponentMapQueryType {
    Render,
    Logic,
}

#[derive(Debug, Clone)]
pub struct QueuedComponent<C> {
    pub id: ComponentId,
    pub component: C,
    pub render_priority: i32,
    pub logic_priority: i32,
}

impl<C> ComponentMap<C> {
    /// Creates a new map.
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            render_priorities: HashMap::new(),
            logic_priorities: HashMap::new(),

            cached_render_ids: Vec::new(),
            cached_logic_ids: Vec::new(),
        }
    }

    /// Returns a reference to the component corresponding to the given ID.
    pub fn get(&self, id: ComponentId) -> Option<&C> {
        self.components.get(&id)
    }

    /// Updates the inner-cached sorted IDs in the map.
    pub fn update_cache(&mut self) {
        let mut cached_render_ids: Vec<_> = self.render_priorities.keys().cloned().collect();
        let priorities = self.priorities(ComponentMapQueryType::Render);
        cached_render_ids.sort_unstable_by_key(|s| *priorities.get(s).unwrap());

        let mut cached_logic_ids: Vec<_> = self
            .logic_priorities
            .iter()
            .filter_map(|(k, v)| (*v != NO_LOGIC_PRIORITY).then_some(*k))
            .collect();

        let priorities = self.priorities(ComponentMapQueryType::Logic);
        cached_logic_ids.sort_unstable_by_key(|s| *priorities.get(s).unwrap());

        self.cached_logic_ids = cached_logic_ids;
        self.cached_render_ids = cached_render_ids;
    }

    /// Inserts a new component in this map.
    ///
    /// Make sure to call [`ComponentMap::update_cache`] once after adding some desired elements.
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

    /// Inserts a new queued component in this map.
    ///
    /// Make sure to call [`ComponentMap::update_cache`] once after adding some desired elements.
    pub fn insert_queued(&mut self, queued: QueuedComponent<C>) {
        self.insert(
            queued.id,
            queued.component,
            queued.render_priority,
            queued.logic_priority,
        );
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

    fn sorted_ids(&self, ty: ComponentMapQueryType) -> &[ComponentId] {
        match ty {
            ComponentMapQueryType::Render => &self.cached_render_ids,
            ComponentMapQueryType::Logic => &self.cached_logic_ids,
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
        self.sorted_ids(ty)
            .iter()
            .map(|i| (*i, &self.components[i]))
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the highest-prioritized component, where
    /// each item is a reference to a component.
    pub fn descending_iter(
        &self,
        ty: ComponentMapQueryType,
    ) -> impl Iterator<Item = (ComponentId, &C)> {
        self.sorted_ids(ty)
            .iter()
            .map(|i| (*i, &self.components[i]))
            .rev()
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the lowest-prioritized component, where
    /// each item is a mutable reference to a component.
    pub fn ascending_iter_mut(
        &mut self,
        ty: ComponentMapQueryType,
        mut f: impl FnMut(ComponentId, &mut C),
    ) {
        let cloned: Vec<_> = self.sorted_ids(ty).to_vec();
        for id in cloned {
            f(id, self.components.get_mut(&id).unwrap());
        }
    }

    /// Provides an [`Iterator`] with the provided priority type.
    ///
    /// This iterator starts from the highest-prioritized component, where
    /// each item is a mutable reference to a component.
    pub fn descending_iter_mut(
        &mut self,
        ty: ComponentMapQueryType,
        mut f: impl FnMut(ComponentId, &mut C),
    ) {
        let cloned: Vec<_> = self.sorted_ids(ty).iter().cloned().rev().collect();
        for id in cloned {
            f(id, self.components.get_mut(&id).unwrap());
        }
    }
}

impl<C> Default for ComponentMap<C> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ComponentMapIter<'a, C>(Iter<'a, ComponentId, C>);

impl<'a, C> Iterator for ComponentMapIter<'a, C> {
    type Item = (ComponentId, &'a C);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id, component)| (*id, component))
    }
}

impl<'a, C> IntoIterator for &'a ComponentMap<C> {
    type Item = (ComponentId, &'a C);

    type IntoIter = ComponentMapIter<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        ComponentMapIter(self.components.iter())
    }
}

pub struct ComponentMapIterMut<'a, C>(IterMut<'a, ComponentId, C>);

impl<'a, C> Iterator for ComponentMapIterMut<'a, C> {
    type Item = (ComponentId, &'a mut C);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(id, component)| (*id, component))
    }
}

impl<'a, C> IntoIterator for &'a mut ComponentMap<C> {
    type Item = (ComponentId, &'a mut C);

    type IntoIter = ComponentMapIterMut<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        ComponentMapIterMut(self.components.iter_mut())
    }
}

/// A trait for something that can be "held", like a button.
pub trait Hold {
    const MAX_HOLD_TIME: u32;

    fn hold_time(&self) -> u32;
    fn set_hold_time(&mut self, new_time: u32);

    fn update_hold_time(&mut self, delta: u128, held: bool) {
        let hold_time = self.hold_time();
        if held {
            self.set_hold_time(
                (self.hold_time() as u128 + delta).min(Self::MAX_HOLD_TIME as u128) as u32,
            );
        } else {
            self.set_hold_time((hold_time as u128).saturating_sub(delta) as u32);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    LevelGo(Instant),
    LevelFinish(u32),
}
