use crate::stream::stage::demand::Demand;

use super::sa;
use crate::stream::stage::graph::GraphStage;
use crate::stream::stage::shape::ShapeType;
use multiqueue::{broadcast_queue, BroadcastReceiver, BroadcastSender};
use std::mem;
use std::ops::Deref;

pub struct Architect {
    demand_tx: BroadcastSender<Demand>,
    demand_rx: BroadcastReceiver<Demand>,

    stages: Vec<Box<dyn GraphStage>>,
}

impl Architect {
    pub fn graph(stages: Vec<Box<dyn GraphStage>>) -> Architect {
        let stage_count = stages.len();
        let (demand_tx, demand_rx) = broadcast_queue(stage_count as u64);

        Architect {
            demand_rx,
            demand_tx,
            stages,
        }
    }

    pub fn run(&mut self) -> &Self {
        self.visit_stages().check_bounds();
        unimplemented!()
    }

    fn check_bounds(&mut self) -> &mut Self {
        assert_eq!(self.stages.first().unwrap().get_shape(), ShapeType::Source);

        assert_eq!(self.stages.last().unwrap().get_shape(), ShapeType::Sink);

        self
    }

    fn visit_stages(&mut self) -> &mut Self {
        let tx = self.demand_tx.clone();
        let rx = self.demand_rx.add_stream();

        self.stages.iter_mut().for_each(|stage| {
            stage.build_demand(tx.clone(), rx.clone());
        });

        self
    }
}
