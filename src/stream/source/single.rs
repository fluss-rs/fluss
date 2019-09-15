use crate::stream::stage::prelude::*;
use crate::stream::source::macros::*;
use futures::io::Error;

pub struct Single<'a, O> {
    pub shape: Box<dyn Shape<'a, NotUsed, O>>,
    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic
}

impl<'a, I, O> GraphStage<'a, I, O> for Single<'a, O>
    where
        O: Clone {
    fn shape(&mut self) -> Box<dyn Shape<'a, I, O>> {
        let single_source_outlet=
            Outlet::<O>::new(0, "Single.out");
        self.shape = SourceShape::new_from(single_source_outlet);
        self.shape.clone()
    }

    fn in_handler(&mut self) -> Box<dyn InHandler> {
        unimplemented!()
    }

    fn out_handler(&mut self) -> Box<dyn OutHandler> {
        #[derive(Clone, Debug)]
        struct SingleOutHandler();
        impl OutHandler for SingleOutHandler {
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

        self.out_handler = Box::new(SingleOutHandler());

        self.out_handler.clone()
    }

    fn create_logic(&mut self, attributes: Attributes) -> GraphStageLogic {
        unimplemented!()
    }
}
