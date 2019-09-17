use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;

pub struct Map<I, O> {
    pub shape: FlowShape<'static, I, O>,
    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

#[derive(Clone, Debug)]
struct MapInHandler<I> {
    //    elem: I,
    pub rx: Receiver<I>,
    pub tx: Sender<I>,
}

#[derive(Clone, Debug)]
struct MapOutHandler<O> {
    //    elem: O,
    pub rx: Receiver<O>,
    pub tx: Sender<O>,
}

impl<'a, I, O> GraphStage<'a, I, O> for Map<I, O>
where
    I: Clone + 'static,
    O: Clone + 'static,
{
    fn build_shape(&mut self) {
        let map_flow_inlet = Inlet::<I>::new(0, "Map.in");
        let map_flow_outlet = Outlet::<O>::new(0, "Map.out");

        self.shape = FlowShape {
            inlet: map_flow_inlet,
            outlet: map_flow_outlet,
        };
    }

    fn in_handler(&mut self) -> Box<dyn InHandler> {
        impl<I> InHandler for MapInHandler<I>
        where
            I: Clone + 'static,
        {
            fn name(&self) -> String {
                unimplemented!()
            }

            fn on_push(&self) {
                unimplemented!()
            }

            fn on_upstream_finish(&self) {
                unimplemented!()
            }

            fn on_upstream_failure(&self, err: Error) {
                unimplemented!()
            }
        }

        let (tx, rx) = unbounded::<I>();
        self.in_handler = Box::new(MapInHandler { tx, rx });

        self.in_handler.clone()
    }

    fn out_handler(&mut self) -> Box<dyn OutHandler> {
        impl<O> OutHandler for MapOutHandler<O>
        where
            O: Clone + 'static,
        {
            fn name(&self) -> String {
                String::from("single-source")
            }

            fn on_pull(&self) {
                unimplemented!()
            }

            fn on_downstream_finish(&self) {
                unimplemented!()
            }

            fn on_downstream_finish_explicit(&self, err: Error) {
                unimplemented!()
            }
        }

        let (tx, rx) = unbounded::<O>();
        self.out_handler = Box::new(MapOutHandler { tx, rx });

        self.out_handler.clone()
    }

    fn create_logic(&mut self, attributes: Attributes) -> GraphStageLogic {
        self.build_shape();
        self.in_handler();
        self.out_handler();

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<I, O>(shape);
        gsl.set_inlet_handler(self.shape.inlet.clone(), self.in_handler.clone());
        gsl.set_outlet_handler(self.shape.outlet.clone(), self.out_handler.clone());
        self.logic = gsl.clone();
        gsl
    }
}
