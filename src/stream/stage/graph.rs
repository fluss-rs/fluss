use std::marker::PhantomData as marker;

use crate::stream::stage::attributes::Attributes;
use crate::stream::stage::handlers::*;
use crate::stream::stage::lets::{Inlet, Outlet};
use crate::stream::stage::shape::Shape;

pub struct GraphStage<'a, I, O> {
    pub shape: Box<dyn Shape<'a, I, O>>,
    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
}

impl<'a, I, O> GraphStage<'a, I, O> {
    fn create_logic(&self, attributes: Attributes) {
        unimplemented!()
    }
}

///////////////
// Graph Stage Logic
///////////////
pub struct GraphStageLogic {
    pub in_count: usize,
    pub out_count: usize,
    pub handlers: Vec<Box<dyn Handler>>,
}

impl GraphStageLogic {
    pub fn new(in_count: usize, out_count: usize) -> Self {
        GraphStageLogic {
            in_count,
            out_count,
            handlers: Vec::with_capacity(in_count + out_count),
        }
    }

    pub fn new_with_shape<I, O>(shape: Box<dyn Shape<I, O>>) -> Self {
        GraphStageLogic {
            in_count: shape.inlets().len(),
            out_count: shape.outlets().len(),
            handlers: Vec::with_capacity(shape.inlets().len() + shape.outlets().len()),
        }
    }

    pub fn set_inlet_handler<I: Clone>(&mut self, inlet: Inlet<I>, handler: Box<dyn Handler>) {
        let inlet_handler = &[handler];
        self.handlers
            .splice(inlet.id..inlet.id, inlet_handler.iter().cloned());
    }

    pub fn set_outlet_handler<O: Clone>(&mut self, outlet: Outlet<O>, handler: Box<dyn Handler>) {
        let outlet_handler = &[handler];
        self.handlers
            .splice(outlet.id..outlet.id, outlet_handler.iter().cloned());
    }
}
