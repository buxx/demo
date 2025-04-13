use archives::{ArchiverItem, ArchivesComponent};
use common::{Component, FromComponentItem, IntoOtherComponentItem, System};
use users::{UsersComponents, UsersItem};

macro_rules! items {
    ( $( [$name:ident, $event:ident] ),* ) => {
        enum ComponentItem {
            $( $name($event) ),*
        }

        $(
            impl FromComponentItem<$event> for ComponentItem {
                fn from_component_item(value: $event) -> Self {
                    ComponentItem::$name(value)
                }
            }
        )*
    };
}

items!(
    [UsersComponents, UsersItem],
    [ArchivesComponent, ArchiverItem]
);

// TODO: procedural macro ?
impl IntoOtherComponentItem<archives::SystemEvent> for ComponentItem {
    fn into_other_component_item(&self) -> Option<archives::SystemEvent> {
        match self {
            ComponentItem::ArchivesComponent(_item) => None,
            ComponentItem::UsersComponents(item) => match item {
                UsersItem::CreatedUser(user) => Some(archives::SystemEvent::Users(
                    archives::UsersSystemEvent::Created(user.clone()),
                )),
            },
        }
    }
}

fn main() {
    let users = UsersComponents;
    let archives = ArchivesComponent;
    let c: Vec<Box<dyn Component<ComponentItem>>> = vec![Box::new(users), Box::new(archives)];
    let system: System<ComponentItem> = System { components: c };
    system.work();
}
