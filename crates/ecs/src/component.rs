use std::any::{type_name_of_val, Any};

pub trait Component {
    fn type_name(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Component for T {
    fn type_name(&self) -> String {
        type_name_of_val(self).to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
