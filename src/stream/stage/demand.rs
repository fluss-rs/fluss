use crossbeam_channel::{Sender, Receiver};

#[derive(Clone)]
pub enum DemandStyle {
    DemandFull(usize),
    DemandPartial(usize, usize)
}

#[derive(Clone)]
pub struct Demand {
    pub stage_id: usize,
    pub style: DemandStyle
}

// Demand endpoint struct
#[derive(Clone)]
pub struct Demander {
    pub tx: Sender<Demand>,
    pub rx: Receiver<Demand>
}
