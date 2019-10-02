use crate::stream::source::macros::*;
use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;

pub struct Single<O> {
    pub shape: SourceShape<'static, O>,

    pub elem: O,

    pub demand_rx: Receiver<Demand>,
    pub demand_tx: Sender<Demand>,

    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

#[derive(Clone, Debug)]
struct SingleOutHandler<O> {
    elem: O,
    pub rx: Receiver<O>,
    pub tx: Sender<O>,
}

impl<'a, O> GraphStage<'a> for Single<O>
where
    O: Clone +  'static,
{
    fn build_shape(&mut self) {
        let single_source_outlet = Outlet::<O>::new(0, "Single.out");
        self.shape = SourceShape {
            outlet: single_source_outlet,
        };
    }

    fn build_in_handler(&mut self) -> Box<dyn InHandler> {
        unimplemented!()
    }

    fn build_out_handler(&mut self) -> Box<dyn OutHandler> {
        impl<O> OutHandler for SingleOutHandler<O>
        where
            O: Clone + 'static,
        {
            fn name(&self) -> String {
                String::from("single-source-out")
            }

            fn on_pull(&self) {
                self.tx.send(self.elem.clone());
                self.on_downstream_finish();
            }

            fn on_downstream_finish(&self) {
                // TODO: Signal stop to the runtime (architect)
            }

            fn on_downstream_finish_explicit(&self, err: Error) {
                unimplemented!()
            }
        }

        let (tx, rx) = unbounded();

        self.out_handler = Box::new(SingleOutHandler {
            elem: self.elem.clone(),
            tx,
            rx,
        });

        self.out_handler.clone()
    }

    fn build_demand(&'a mut self, tx: Sender<Demand>, rx: Receiver<Demand>) {
        self.demand_tx = tx;
        self.demand_rx = rx;
    }

    fn create_logic(&mut self, attributes: Attributes) -> GraphStageLogic {
        self.build_shape();
        self.build_out_handler();

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<NotUsed, O>(shape);
        gsl.set_outlet_handler(self.shape.outlet.clone(), self.out_handler.clone());
        self.logic = gsl.clone();
        gsl
    }
}
