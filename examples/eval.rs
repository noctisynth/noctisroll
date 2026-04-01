use noctisroll::prelude::*;

fn main() {
    let res = eval_dice_expression("1d0".to_string()).unwrap();
    println!("Total: {}", res.total);
}
