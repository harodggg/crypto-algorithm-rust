#[derive(Debug)]
enum ChachaBuf{
    U([u32;16]),   
    C([u8;64])
}

trait Round { 
    fn quarter_round(a: [u8;4],b: [u8;4],c: [u8;4],d: [u8;4]){ 
        unimplemented!();
    }
}
trait Decoder { 
    fn decode() { 
    }
}

trait Encoder { 
    fn encode() { 
    }
}
fn main() {
    let chachabuf_test: ChachaBuf = ChachaBuf::U([0;16]);
    println!("Hello, world!{:?}",chachabuf_test);
}
