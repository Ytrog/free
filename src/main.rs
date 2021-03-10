use sysinfo::*;

fn main() {
    let s = System::new_all();
    let disks = s.get_disks();
    for d in disks {
        println!("{:?}", d);
    }
}
