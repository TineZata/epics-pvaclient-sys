use epics_pvaclient_sys::get_pv;

fn main() {
    let pv_name = "rec:X";
    let value = get_pv(&pv_name);
    println!("{}", value);
}
