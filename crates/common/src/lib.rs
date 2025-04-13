pub mod user;
pub struct System<T> {
    pub components: Vec<Box<dyn Component<T>>>,
}

impl<T> System<T> {
    pub fn work(&self) {
        let work: Vec<Item<T>> = self.components.iter().map(|c| c.work()).collect();
        for component in &self.components {
            for item in &work {
                component.react(item)
            }
        }
    }
}

pub enum Item<T> {
    Component(T),
}

pub trait Component<T> {
    fn work(&self) -> Item<T>;
    fn react(&self, item: &Item<T>);
}

pub trait FromComponentItem<T> {
    fn from_component_item(value: T) -> Self;
}

pub trait IntoOtherComponentItem<T> {
    fn into_other_component_item(&self) -> Option<T>;
}

// pub trait IntoItem<T> {
//     fn into_item(self) -> Item<T>;
// }
