mod varnmala;
mod varn;
use varnmala::Varnmala;
use varn::Varn;

fn main() {
    let mut varnmala = Varnmala::new();

    varnmala.initialize_varns();

    varnmala.print_all_varns();

    let varna = Varn::from_char('क');
    varna.print();
}
