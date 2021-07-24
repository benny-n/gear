
extern crate component_macro;
pub use component_macro::Component;
use std::any::Any;
pub trait Component{
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}



