use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;
use objekt_clonable::clonable;
use std::marker::PhantomData;

#[clonable]
pub trait MapClosure<I, O>: Fn(I) -> O + Clone + Send + Sync + 'static {}
type MapFn<I, O> = Box<dyn MapClosure<I, O>>;

pub struct Map<I, O> {
    pub shape: FlowShape<'static, I, O>,

    pub map_fn: MapFn<I, O>,

    pub demand_rx: Receiver<Demander>,
    pub demand_tx: Sender<Demander>,

    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

#[derive(Clone)]
struct MapHandler<I, O> {
    map_fn: Option<MapFn<I, O>>,

    pub in_rx: Option<Receiver<I>>,
    pub in_tx: Option<Sender<I>>,

    pub demand_rx: Option<Receiver<Demander>>,
    pub demand_tx: Option<Sender<Demander>>,

    pub out_rx: Option<Receiver<O>>,
    pub out_tx: Option<Sender<O>>,
}

impl<I, O> Default for MapHandler<I, O> {
    fn default() -> Self {
        MapHandler {
            map_fn: None,

            in_rx: None,
            in_tx: None,

            demand_rx: None,
            demand_tx: None,

            out_tx: None,
            out_rx: None,
        }
    }
}

impl<'a, I, O> GraphStage<'a> for Map<I, O>
where
    I: Clone + Send + Sync + 'static,
    O: Clone + Send + Sync + 'static,
{
    fn build_shape(&mut self) {
        let map_flow_inlet = Inlet::<I>::new(0, "Map.in");
        let map_flow_outlet = Outlet::<O>::new(0, "Map.out");

        self.shape = FlowShape {
            inlet: map_flow_inlet,
            outlet: map_flow_outlet,
        };
    }

    fn build_in_handler(&mut self) -> Box<dyn InHandler> {
        impl<I, O> InHandler for MapHandler<I, O>
        where
            I: Clone + Send + Sync,
            O: Clone + Send + Sync,
        {
            fn name(&self) -> String {
                String::from("map-flow-in")
            }

            fn on_push(&self) {
                unimplemented!();
                if let Ok(elem) = self.in_rx.unwrap().try_recv() {
                    let resp: O = self.map_fn.as_ref().unwrap()(elem);
                    self.out_tx.unwrap().send(resp);
                } else {
                    // todo: handle error case of try_recv
                    // todo: on_pull make demand from the upper
                    let demand = Demand {

                    };
                    self.demand_tx.send
                }
            }

            fn on_upstream_finish(&self) {
                unimplemented!()
            }

            fn on_upstream_failure(&self, err: Error) {
                unimplemented!()
            }
        }

        Box::new(MapHandler::<I, O>::default())
    }

    fn build_out_handler(&mut self) -> Box<dyn OutHandler> {
        impl<I, O> OutHandler for MapHandler<I, O>
        where
            I: Clone + Send + Sync + 'static,
            O: Clone + Send + Sync + 'static,
        {
            fn name(&self) -> String {
                String::from("map-flow-out")
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

        Box::new(MapHandler::<I, O>::default())
    }

    fn build_demand(&'a mut self, tx: Sender<Demander>, rx: Receiver<Demander>) {
        self.demand_tx = tx;
        self.demand_rx = rx;
    }

    fn create_logic(&mut self, attributes: Attributes) -> GraphStageLogic {
        self.build_shape();
        self.build_in_handler();
        self.build_out_handler();

        let (in_tx, in_rx) = unbounded::<I>();
        let (out_tx, out_rx) = unbounded::<O>();

        let handler = Box::new(MapHandler {
            map_fn: Some(self.map_fn.clone()),
            in_tx: Some(in_tx),
            in_rx: Some(in_rx),
            out_rx: Some(out_rx),
            out_tx: Some(out_tx),
        });

        self.in_handler = handler.clone();
        self.out_handler = handler.clone();

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<I, O>(shape);
        gsl.set_inlet_handler(self.shape.inlet.clone(), self.in_handler.clone());
        gsl.set_outlet_handler(self.shape.outlet.clone(), self.out_handler.clone());
        self.logic = gsl.clone();
        gsl
    }
}
