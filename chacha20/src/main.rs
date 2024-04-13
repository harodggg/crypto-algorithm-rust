use rand::Rng;


const CONSTANTS: &[u8] = "expand 32-byte k".as_bytes();
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
    let mut rng = rand::thread_rng();
    let mut nonce = [0u8;16];
    let mut key = [0u8;32];
    rng.fill(&mut nonce);
    rng.fill(&mut key);
    
    println!("Hello, world!{:?}",key.len());
}


#[cfg(test)] 
mod tests { 
    use super::*;

    #[test]
    fn one_test() { 
        assert_eq!("aa","aa")
    }
}