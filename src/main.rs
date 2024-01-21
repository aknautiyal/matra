mod varnmala;
mod varn;
mod shabd;
use varnmala::Varnmala;
use varn::Varn;
use varn::VarnList;
use shabd::Shabd;

fn main() {
    let mut varnmala = Varnmala::new();

    varnmala.initialize_varns();

    varnmala.print_all_varns();

    let mut varna: Varn;

    let mut shabd = Shabd::new();
    //let mut varnlist = VarnList::new();
    varna = Varn::from_char('र');
    //varnlist.push(varna);
    shabd.base.push(varna);
    varna = Varn::from_char('ा');
    //varnlist.push(varna);
    shabd.base.push(varna);
    varna = Varn::from_char('ऽ');
    shabd.base.push(varna);
    varna = Varn::from_char('म');
    //varnlist.push(varna);
    shabd.base.push(varna);
    varna = Varn::from_char('न');
    shabd.base.push(varna);
    varna = Varn::from_char('्');
    shabd.base.push(varna);
    varna = Varn::from_char('त');
    shabd.base.push(varna);
    varna = Varn::from_char('्');
    shabd.base.push(varna);
    varna = Varn::from_char('न');
    shabd.base.push(varna);
    varna = Varn::from_char('ा');
    shabd.base.push(varna);
    //varnlist.print_varns();
    /*
    if let Some(x) = varnlist.pop() {
        x.print();
    }
    */
    shabd.make_akshars();
    shabd.print();
}
