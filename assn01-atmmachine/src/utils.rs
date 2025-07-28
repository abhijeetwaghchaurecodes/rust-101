
use crate::cash::Cash;

pub fn print_stack_and_heap_info(id: u32, cash: &Cash) {
    println!("[Memory Info]");
    println!("  -> ATM ID stack addr: {:p}", &id);
    println!("  -> Cash object heap addr: {:p}", &cash);
}
