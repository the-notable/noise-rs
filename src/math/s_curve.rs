pub(crate) mod cubic;
pub(crate) mod quintic;

pub trait SCurve {}

impl<T> SCurve for T {}
