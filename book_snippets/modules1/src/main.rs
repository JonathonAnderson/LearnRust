use crate::garden::vegetables::Asparagus;

mod garden;

fn main() {
    let crop1 = crate::garden::vegetables::Zucchini;
    let crop2 = Asparagus;

    println!("I'm growing {crop1:?}");
    println!("I'm also growing {crop2:?}");
}