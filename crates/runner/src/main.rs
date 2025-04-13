use archives::{ArchiverItem, ArchivesComponent};
use common::{Component, FromComponentItem, AsOtherComponentItem, System};
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

macro_rules! convert {
    ($target_type:ty, $outer_variant:path, { $($inner_pattern:pat => $result:expr),* $(,)? }) => {
        impl AsOtherComponentItem<$target_type> for ComponentItem {
            fn as_other_component_item(&self) -> Option<$target_type> {
                #[allow(unreachable_patterns)]
                match self {
                    ComponentItem::ArchivesComponent(_) => None,
                    $outer_variant(inner) => match inner {
                        $(
                            $inner_pattern => Some($result),
                        )*
                        _ => None,
                    },
                    _ => None,
                }
            }
        }
    };
}

items!(
    [UsersComponents, UsersItem],
    [ArchivesComponent, ArchiverItem]
);


convert!(
    archives::SystemEvent,
    ComponentItem::UsersComponents,
    {
        UsersItem::CreatedUser(user) => archives::SystemEvent::Users(
            archives::UsersSystemEvent::Created(user.clone())
        ),
        UsersItem::DeletedUser(user) => archives::SystemEvent::Users(
            archives::UsersSystemEvent::Deleted(user.clone())
        )
    }
);

fn main() {
    let users = UsersComponents;
    let archives = ArchivesComponent;
    let c: Vec<Box<dyn Component<ComponentItem>>> = vec![Box::new(users), Box::new(archives)];
    let system: System<ComponentItem> = System { components: c };
    system.work();
}
