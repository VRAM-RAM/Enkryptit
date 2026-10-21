use crate::types::{CompressionType, KeyParams, ParallelismType};
pub mod inspection;
pub mod params_helpers;
pub mod treatment;



#[macro_export]
macro_rules! params_changed {
    () => {
        println!("\n Params were changed ! \n")
    };
}

pub fn show_params(kt: &KeyParams, c: &CompressionType, p: &ParallelismType) {
    println!();
    println!(
        "Actual parameters : \n KeyType : {:?} \n Compression : {:?} \n Parallelism Option : {:?}",
        kt, c, p
    );
    println!();
}
