use diva_livomo::foliate;

fn main() {
    let fos = foliate::load().unwrap();
    for item in fos.into_iter() {
        if item.has_annotation() {
            println!("{}", item.to_md());
        }
    }
}
