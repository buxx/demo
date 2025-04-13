use common::{user::User, Component, FromComponentItem, Item};

pub struct UsersComponents;

impl<T: FromComponentItem<UsersItem>> Component<T> for UsersComponents {
    fn work(&self) -> Item<T> {
        Item::Component(T::from_component_item(UsersItem::CreatedUser(User(
            "Frnck".into(),
        ))))
    }

    fn react(&self, _item: &Item<T>) {}
}

pub enum UsersItem {
    CreatedUser(User),
}
