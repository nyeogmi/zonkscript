use std::{borrow::Cow};

// TODO: Allow merging two ModuleBuilders, and generate linker errors if needed
use super::*;

impl ModuleBuilder {
    pub fn build(mut self) -> Module {
        let std_primitives = self.get_std_primitives_record();

        // == generate a struct for every procedure's locals ==
        let proc_ids = 0..self.procedures.len();
        for proc_id in proc_ids {
            // NOTE: This skips the module methods for borrowing reasons. Kinda hacky!
            match self.procedures.mutate_raw(ZId::new(proc_id)) {
                Phased::Mentioned => {}
                Phased::InProgress(proc) | Phased::Sealed(proc) => {
                    // TODO: Don't bother generating a symbol.
                    let frame = self.datatypes.reference( &Cow::Owned(format!("internal!scope!{:?}", proc_id)));
                    for local in proc.locals.data.iter() {
                        // TODO: Assert local_ids are consecutive
                        self.datatypes.mutate(frame, DataTypeBuilder::new).unwrap().push(local.0);
                    }
                    self.datatypes.seal(frame);
                    proc.frame_hint = Some(frame);
                }
                Phased::Built(_) => {}
            }
        }

        // == now create object ==
        let procedures: Finalized<Procedure> = self.procedures.link(); 
        let datatypes: Finalized<DataType> = self.datatypes.link(); 
        let primitives: Finalized<Primitive> = self.primitives.link();


        Module { 
            procedures, 
            datatypes,
            primitives,
            std_primitives,
        }
    }
}