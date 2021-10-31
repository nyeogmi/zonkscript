use std::{borrow::Cow, collections::{BTreeMap, BTreeSet}};

// TODO: Allow merging two ModuleBuilders, and generate 
// linker errors if needed
// TODO: Error or warn if something is not finalized 
use crate::reexports::*;
use super::*;

impl ModuleBuilder {
    pub fn build(mut self) -> Module {
        // == generate std primitives object ==
        let std_primitives = self.get_std_primitives_record();

        // == generate a structure for every procedure's locals ==
        // TODO: Don't clone
        let mut proc_frame = BTreeMap::new();
        let proc_in_progress = self.procedures.in_progress.clone();
        for (proc_id, proc) in proc_in_progress {
            // TODO: Don't bother generating a symbol.
            let frame = self.structure( &Cow::Owned(format!("internal!scope!{:?}", proc_id)));
            for (local_id, local) in proc.locals.data {
                // TODO: Assert local_ids are consecutive
                self.push_field(frame, local.0);
            }
            self.seal_structure(frame);
            proc_frame.insert(proc_id, frame);
        }

        // == now create object ==
        let mut procedures: Finalized<Procedure> = Finalized {
            names: self.procedures.names,
            data: self.procedures.data,
        };

        let mut structs: Finalized<Struct> = Finalized {
            names: self.structs.names,
            data: self.structs.data,
        };

        let primitives: Finalized<Primitive> = Finalized { 
            names: self.primitives.names, 
            data: self.primitives.data,
        };

        // now go ahead and finish building everything!
        // TODO: Abstract around the copy-pasted code here

        // == procedures ==
        let mut procedures_finalized = self.procedures.finalized;
        let mut procedures_in_progress = self.procedures.in_progress;

        fn finalize_procedure(
            id: Id<Procedure>, 
            finalized: &mut BTreeSet<Id<Procedure>>,
            proc_frame: &BTreeMap<Id<Procedure>, Id<Struct>>,
            in_progress: &mut BTreeMap<Id<Procedure>, ProcedureBuilder>,
            data: &mut RawPom<Procedure>,
        ) {
            let builder = if let Some(x) = in_progress.remove(&id) {
                x
            } else { return; };

            if finalized.contains(&id) { return; }

            let proc = builder.build(*proc_frame.get(&id).unwrap());

            *data.get_mut(id).unwrap() = proc;
            finalized.insert(id);
        }

        while let Some((id, _)) = procedures_in_progress.iter().next() {
            finalize_procedure(*id, &mut procedures_finalized, &proc_frame, &mut procedures_in_progress, &mut procedures.data);
        }

        // == structs ==
        let mut structs_finalized = self.structs.finalized;
        let mut structs_in_progress = self.structs.in_progress;

        // TODO: Detect cycles and don't recurse
        fn finalize_struct(
            id: Id<Struct>,
            finalized: &mut BTreeSet<Id<Struct>>,
            in_progress: &mut BTreeMap<Id<Struct>, StructBuilder>,
            data: &mut RawPom<Struct>,
        ) {
            let builder = if let Some(x) = in_progress.remove(&id) {
                x
            } else { return; };

            if finalized.contains(&id) { return; }

            let new = builder.build(
                &mut |struct_id, cb: &mut dyn FnMut(&Struct)| { 
                    finalize_struct(struct_id, finalized, in_progress, data);
                    cb(data.get(struct_id).unwrap()) 
                }
            );
            *data.get_mut(id).unwrap() = new;
            finalized.insert(id);
        }

        while let Some((id, _)) = structs_in_progress.iter().next() {
            finalize_struct(*id, &mut structs_finalized, &mut structs_in_progress, &mut structs.data);
        }

        Module { 
            procedures, 
            structs,
            primitives,
            std_primitives,
        }
    }
}