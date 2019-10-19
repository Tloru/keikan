use std::rc::Rc;

use crate::structures::camera::Camera;
use crate::objects::traits::{ March, Trace };

pub struct Scene {
    pub march: Vec<Rc<dyn March>>,
    pub trace: Vec<Rc<dyn Trace>>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene { march: vec![], trace: vec![], camera: camera }
    }

    pub fn add_march(&mut self, march: impl March + 'static) {
        self.march.push(Rc::new(march));
    }

    pub fn add_trace(&mut self, trace: impl Trace + 'static) {
        self.trace.push(Rc::new(trace));
    }
}
