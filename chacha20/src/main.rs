#[derive(Debug)]
enum ChachaBuf{
    U([u32;16]),   
    C([u8;64])
}
fn main() {
    let chachabuf_test: ChachaBuf = ChachaBuf::U([0;16]);
    println!("Hello, world!{:?}",chachabuf_test);
}
