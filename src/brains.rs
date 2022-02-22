use crate::cell::CellBrain;





#[derive(Default)]
pub struct DefaultBrain {

}

impl CellBrain for DefaultBrain {
    fn proceed(self: &Self, chromosome: &chromosome::Chromosome<u8>, current_cmd_index: &mut usize) -> bool {
        todo!()
    }
}