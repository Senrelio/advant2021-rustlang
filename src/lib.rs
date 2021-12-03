use std::io::Read;

pub fn get_input(day: u32) -> String {
    let path = format!("./inputs/day{}_input", day);
    let mut s = String::new();
    let mut f = std::fs::File::open(path).unwrap();
    f.read_to_string(&mut s).unwrap();
    s
    // let path = format!("../../inputs/day{}_input", day);
    // include_str!()
}
