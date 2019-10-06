use crate::stream::stage::demand::{Demand};
use crossbeam_channel::{Sender, Receiver, unbounded};
use crate::stream::stage::graph::GraphStage;
use crate::stream::stage::shape::ShapeType;

pub struct Architect<'a> {
    demand_tx: Sender<Demand>,
    demand_rx: Receiver<Demand>,

    stages: Vec<Box<dyn GraphStage<'a>>>
}


impl<'a> Architect<'a> {
    pub fn graph(stages: Vec<Box<dyn GraphStage<'static>>>) -> Architect {
        let (demand_tx, demand_rx) = unbounded::<Demand>();
        Architect {
            demand_rx,
            demand_tx,
            stages
        }
    }

    pub fn run() {
        unimplemented!()
    }

    fn check_bounds(&'a self) {
        if let Some(root) = self.stages.first() {
            if root.get_shape() != ShapeType::Source {
                unimplemented!()
            }
        }
    }

    fn visit_stages(&'a mut self) {
        let tx = self.demand_tx.clone();
        let rx = self.demand_rx.clone();

        self.stages.iter_mut().for_each(|stage| {
            stage.build_demand(tx.clone(), rx.clone())
        });
    }
}
