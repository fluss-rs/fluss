#[cfg(test)]
mod tests {
    use fluss::stream::stage::graph::GraphStageLogic;
    use fluss::stream::stage::handlers::{InHandler, OutHandler};
    use fluss::stream::stage::lets::{Inlet, Outlet};
    use fluss::stream::stage::shape::SourceShape;
    use fluss::stream::stage::shape::*;
    use fluss::stream::stage::types::NotUsed;
    use futures::io::Error;
    use std::any::Any;

    //        let inlet0 = Inlet::<u64>::new(0, "in0");
    //        let inlet1 = Inlet::<u64>::new(1, "in1");
    //
    //        let outlet1 = Outlet::<u64>::new(1, "out1");
    //        let outlet2 = Outlet::<u64>::new(2, "out2");

    #[test]
    fn graph_stage_logic_io_ordering() {
        let outlet0 = Outlet::<u64>::new(0, "out0");
        let mut shape = SourceShape::new_from(outlet0);
        let mut gsl = GraphStageLogic::new_with_shape::<NotUsed, u64>(shape);
        gsl.set_outlet_handler(outlet0, Box::new(RepeaterLogic()));

        #[derive(Clone, Debug)]
        pub struct RepeaterLogic();
        impl OutHandler for RepeaterLogic {
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

        for i in gsl.handlers {
            println!("Taken Handler :: {:?}", i);
        }
    }
}
