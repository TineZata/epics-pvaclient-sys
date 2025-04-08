use epics_pvaclient_sys::get_pv;

fn main() {
    let pv_name = "TEST:PV1";
    let value = get_pv(&pv_name);
    println!("{}", value);
}
