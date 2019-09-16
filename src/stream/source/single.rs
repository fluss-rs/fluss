use crate::stream::source::macros::*;
use crate::stream::stage::prelude::*;
use futures::io::Error;

pub struct Single<O> {
    pub elem: O,

    pub shape: SourceShape<'static, O>,
    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

#[derive(Clone, Debug)]
struct SingleOutHandler<O> {
    elem: O
}

impl<'a, O> GraphStage<'a, NotUsed, O> for Single<O>
where
    O: Clone + 'static,
{
    fn build_shape(&mut self) {
        let single_source_outlet = Outlet::<O>::new(0, "Single.out");
        self.shape = SourceShape { outlet: single_source_outlet };
    }

    fn in_handler(&mut self) -> Box<dyn InHandler> {
        unimplemented!()
    }

    fn out_handler(&mut self) -> Box<dyn OutHandler> {
        impl<O> OutHandler for SingleOutHandler<O>
        where
            O: Clone + 'static,
        {
            fn name(&self) -> String {
                String::from("single-source")
            }

            fn on_pull(&self) {
                self.elem.clone();
            }

            fn on_downstream_finish(&self) {
                unimplemented!()
            }

            fn on_downstream_finish_explicit(&self, err: Error) {
                unimplemented!()
            }
        }

        self.out_handler = Box::new(SingleOutHandler {elem: self.elem.clone()});

        self.out_handler.clone()
    }

    fn create_logic(&mut self, attributes: Attributes) -> GraphStageLogic {
        self.build_shape();
        self.out_handler();

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<NotUsed, O>(shape);
        gsl.set_outlet_handler(self.shape.outlet.clone(), self.out_handler.clone());
        self.logic = gsl.clone();
        gsl
    }
}
