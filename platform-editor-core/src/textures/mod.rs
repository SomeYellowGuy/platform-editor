use crate::level::StarCondition;

pub mod level;

pub struct IconTextures<T> {
    pub flag: T,

    pub collect: T,
    pub time: T,
    pub items: T,
    pub enemies: T,
    pub enemies_left: T,
    pub gravity: T,
}

impl<T> IconTextures<T> {
    pub fn get(&self, condition: Option<&StarCondition>) -> &T {
        match condition {
            None => &self.flag,
            Some(StarCondition::Collect(_)) => &self.collect,
            Some(StarCondition::Time(_)) => &self.time,
            Some(StarCondition::Items(_)) => &self.items,
            Some(StarCondition::Enemies(_)) => &self.enemies,
            Some(StarCondition::EnemiesLeft(_)) => &self.enemies_left,
            Some(StarCondition::Gravity(_)) => &self.gravity,
        }
    }

    pub fn get_mut(&mut self, condition: Option<&StarCondition>) -> &mut T {
        match condition {
            None => &mut self.flag,
            Some(StarCondition::Collect(_)) => &mut self.collect,
            Some(StarCondition::Time(_)) => &mut self.time,
            Some(StarCondition::Items(_)) => &mut self.items,
            Some(StarCondition::Enemies(_)) => &mut self.enemies,
            Some(StarCondition::EnemiesLeft(_)) => &mut self.enemies_left,
            Some(StarCondition::Gravity(_)) => &mut self.gravity,
        }
    }
}
