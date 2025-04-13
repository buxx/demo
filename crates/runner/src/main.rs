use common::{FromComponentItem, IntoOtherComponentItem, System};
use component1::{Component1, Component1Item};
use component2::{Component2, Component2Item};

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

items!([Component1, Component1Item], [Component2, Component2Item]);


impl IntoOtherComponentItem<component1::OtherComponentItem> for ComponentItem {
    fn into_other_component_item(&self) -> Option<component1::OtherComponentItem> {
        match self {
            ComponentItem::Component1(_item) => todo!(),
            ComponentItem::Component2(item) => match item {
                Component2Item::AKindOfWork => Some(component1::OtherComponentItem::Component2AKindOfWork),
            },
        }
    }
}

fn main() {
    let component1 = Component1;
    let component2 = Component2;
    let system: System<ComponentItem> = System {components: vec![Box::new(component1), Box::new(component2)] };
    system.work();
}
