use common::{user::User, Component, FromComponentItem, AsOtherComponentItem, Item};

pub struct ArchivesComponent;

impl<T: FromComponentItem<ArchiverItem> + AsOtherComponentItem<SystemEvent>> Component<T>
    for ArchivesComponent
{
    fn work(&self) -> Item<T> {
        Item::Component(T::from_component_item(ArchiverItem::ReadyToArchive))
    }

    fn react(&self, item: &Item<T>) {
        match item {
            Item::Component(item) => {
                if let Some(item) = item.as_other_component_item() {
                    match item {
                        SystemEvent::Users(UsersSystemEvent::Created(user)) => {
                            println!("Archive user: {}", user.0)
                        },
                        SystemEvent::Users(UsersSystemEvent::Deleted(user)) => {
                            println!("Archive deleted user: {}", user.0)
                        }
                    }
                }
            }
        }
    }
}

pub enum ArchiverItem {
    ReadyToArchive,
}

pub enum SystemEvent {
    Users(UsersSystemEvent),
}

pub enum UsersSystemEvent {
    Created(User),
    Deleted(User),
}
