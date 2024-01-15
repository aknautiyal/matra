mod varnmala;
mod varn;
use varnmala::Varnmala;
use varn::Varn;
use varn::VarnList;

fn main() {
    let mut varnmala = Varnmala::new();

    varnmala.initialize_varns();

    varnmala.print_all_varns();

    let mut varna: Varn;

    let mut varnlist = VarnList::new();
    varna = Varn::from_char('र');
    varnlist.push(varna);
    varna = Varn::from_char('ा');
    varnlist.push(varna);
    varna = Varn::from_char('म');
    varnlist.push(varna);
    varnlist.print_varns();
    if let Some(x) = varnlist.pop() {
        x.print();
    }
}
