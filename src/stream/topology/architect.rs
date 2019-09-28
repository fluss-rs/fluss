use crate::stream::stage::demand::Demander;
use crossbeam_channel::{Sender, Receiver, unbounded};
use crate::stream::stage::graph::GraphStage;

pub struct Architect {
    demand_tx: Sender<Demander>,
    demand_rx: Receiver<Demander>,

    stages: Vec<Box<dyn GraphStage<'static>>>
}


impl Architect {
    pub fn graph(stages: Vec<Box<dyn GraphStage<'static>>>) -> Architect {
        let (demand_tx, demand_rx) = unbounded::<Demander>();
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
        for i in &self.stages {
            unimplemented!()
        }
    }
}
