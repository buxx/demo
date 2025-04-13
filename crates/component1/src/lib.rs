use common::{Component, FromComponentItem, IntoOtherComponentItem, Item};


pub struct Component1;

impl<T: FromComponentItem<Component1Item> + IntoOtherComponentItem<OtherComponentItem>> Component<T> for Component1 {
    fn work(&self) -> Item<T> {
        Item::Component(T::from_component_item(Component1Item::AKindOfWork))
    }
    
    fn react(&self, item: &Item<T>) {
        match item {
            Item::Component(item) => {
                if let Some(item) = item.into_other_component_item() {
                    match item {
                        OtherComponentItem::Component2AKindOfWork => {
                            todo!()
                        }
                    }
                }
            },
        }
        
    }
}

pub enum Component1Item {
    AKindOfWork,
}

pub enum OtherComponentItem {
    Component2AKindOfWork
}