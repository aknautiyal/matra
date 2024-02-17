mod varnmala;
mod varn;
mod shabd;
mod vakya;
use varnmala::Varnmala;
use varn::Varn;
use varn::VarnList;
use shabd::Shabd;
use vakya::Vakya;

fn main() {
    let mut vakyas = Vakya::from_file("src/input.txt").unwrap();
    for vakya in vakyas.iter_mut() {
        vakya.print();
        println!();
    }
}
