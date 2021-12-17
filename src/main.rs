fn main() {
    let path = match std::env::args_os().nth(1) {
        Some(path) => path,
        None => {
            eprintln!("\x1b[91merror\x1b[0m: please specify an obj file");
            return;
        }
    };
    let obj = match obj::Obj::load(path) {
        Ok(obj) => obj,
        Err(e) => {
            eprintln!("\x1b[91merror\x1b[0m: {}", e);
            eprintln!("\x1b[97m note\x1b[0m: invalid obj file");
            return;
        }
    };
    let mut desmos_string = String::from("\\left[");
    for i in obj
        .data
        .objects
        .iter()
        .flat_map(|o| &o.groups)
        .map(|g| &g.polys)
    {
        for poly in i.iter().map(|p| &p.0) {
            desmos_string.push_str("\\operatorname{polygon}\\left(");
            for point_idx in poly.iter().map(|idx| idx.0) {
                desmos_string.push_str("f\\left(");
                let [x, y, z] = obj.data.position[point_idx];
                desmos_string.extend([
                    &x.to_string(),
                    ",",
                    &y.to_string(),
                    ",",
                    &z.to_string(),
                    "\\right),\\ ",
                ]);
            }
            desmos_string.truncate(desmos_string.len() - 3);
            desmos_string.push_str("\\right),");
        }
    }
    desmos_string.pop();
    desmos_string.push_str("\\right]");
    println!("{}", desmos_string);
}
