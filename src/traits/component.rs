use std::sync::Arc;

use crate::models::components::{Component};



pub  trait IntoComponent {
    fn into_component(self) -> Arc<dyn Component>;
}

pub trait IntoTwilight<T> {
    fn into_twilight(self) -> T;
}