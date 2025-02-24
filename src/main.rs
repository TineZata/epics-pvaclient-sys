use epics_pvaclient_sys::get_pv;

fn main() {
    let pv_name = std::env::args().nth(1).expect("Usage: cargo run <pvname>");
    let value = get_pv(&pv_name);
    println!("{}", value);
}
