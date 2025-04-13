use common::{Component, FromComponentItem, Item};


pub struct Component2;

impl<T: FromComponentItem<Component2Item>> Component<T> for Component2 {
    fn work(&self) -> Item<T> {
        Item::Component(T::from_component_item(Component2Item::AKindOfWork))
    }
    
    fn react(&self, _item: &Item<T>) {
        // if let Some(item) = item.into() {
        //     match item {
        //         Component2Item::AKindOfWork => {
        //             todo!()
        //         }
        //     }
        // }
        todo!()
    }
}

pub enum Component2Item {
    AKindOfWork,
}