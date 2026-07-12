use crate::models::components::ComponentType;



pub  trait IntoComponent {
    fn into_component(self) -> ComponentType;
}

pub trait IntoTwilight<T> {
    fn into_twilight(self) -> T;
}