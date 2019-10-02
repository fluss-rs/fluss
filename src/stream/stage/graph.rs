use std::marker::PhantomData as marker;

use crate::stream::stage::attributes::Attributes;
use crate::stream::stage::handlers::*;
use crate::stream::stage::lets::{Inlet, Outlet};
use crate::stream::stage::shape::Shape;
use crossbeam_channel::{Sender, Receiver};
use crate::stream::stage::demand::{Demander, Demand};

pub trait GraphStage<'a> {
    fn build_shape(&'a mut self);
    fn build_in_handler(&'a mut self) -> Box<dyn InHandler>;
    fn build_out_handler(&'a mut self) -> Box<dyn OutHandler>;
    fn build_demand(&'a mut self, tx: Sender<Demand>, rx: Receiver<Demand>);
    fn create_logic(&'a mut self, attributes: Attributes) -> GraphStageLogic;
}

///////////////
// Graph Stage Logic
///////////////
#[derive(Clone)]
pub struct GraphStageLogic {
    pub in_count: usize,
    pub out_count: usize,
    pub in_handlers: Vec<Box<dyn InHandler>>,
    pub out_handlers: Vec<Box<dyn OutHandler>>,
}

impl GraphStageLogic {
    pub fn new(in_count: usize, out_count: usize) -> Self {
        GraphStageLogic {
            in_count,
            out_count,
            in_handlers: Vec::with_capacity(in_count),
            out_handlers: Vec::with_capacity(out_count),
        }
    }

    pub fn from_shape<I, O>(shape: Box<dyn Shape<I, O>>) -> Self {
        GraphStageLogic {
            in_count: shape.inlets().len(),
            out_count: shape.outlets().len(),
            in_handlers: Vec::with_capacity(shape.inlets().len()),
            out_handlers: Vec::with_capacity(shape.outlets().len()),
        }
    }

    pub fn set_inlet_handler<I: Clone>(&mut self, inlet: Inlet<I>, handler: Box<dyn InHandler>) {
        let inlet_handler = &[handler];
        self.in_handlers
            .splice(inlet.id..inlet.id, inlet_handler.iter().cloned());
    }

    pub fn set_outlet_handler<O: Clone>(
        &mut self,
        outlet: Outlet<O>,
        handler: Box<dyn OutHandler>,
    ) {
        let outlet_handler = &[handler];
        self.out_handlers
            .splice(outlet.id..outlet.id, outlet_handler.iter().cloned());
    }
}
