use std::rc::Rc;
use crate::groups::Group;
use crate::lights::LightSource;
use crate::materials::Material;

pub struct World {
    pub geometry: Box<dyn Group>,
    pub materials: Vec<Rc<dyn Material>>,
    pub lights: Vec<Box<dyn LightSource>>
}

