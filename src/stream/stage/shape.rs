use crate::stream::stage::lets::{Inlet, Outlet};

pub trait Shape<'a, I, O> {
    fn inlets(&self) -> Vec<Inlet<'a, I>>;
    fn outlets(&self) -> Vec<Outlet<'a, O>>;
}

////////////////
// Source Shape
////////////////
pub struct SourceShape<'a, O> {
    pub out: Outlet<'a, O>,
}

impl<'a, I, O> Shape<'a, I, O> for SourceShape<'a, O>
where
    O: Clone,
{
    fn inlets(&self) -> Vec<Inlet<'a, I>> {
        Vec::new()
    }

    fn outlets(&self) -> Vec<Outlet<'a, O>> {
        vec![self.out.clone()]
    }
}

////////////////
// Flow Shape
////////////////

////////////////
// Sink Shape
////////////////
