use crate::stream::stage::demand::{Demand};
use crossbeam_channel::{Sender, Receiver, unbounded};
use crate::stream::stage::graph::GraphStage;

pub struct Architect {
    demand_tx: Sender<Demand>,
    demand_rx: Receiver<Demand>,

    stages: Vec<Box<dyn GraphStage<'static>>>
}


impl Architect {
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

    fn visit_stages(&mut self) {
        unimplemented!();
//        for &mut stage in *self.stages {
//            stage.build_demand(
//                self.demand_tx.clone(),
//                self.demand_rx.clone()
//            )
//        }
    }
}
