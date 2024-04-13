use rand::Rng;


const CONSTANTS: &[u8] = "expand 32-byte k".as_bytes();
#[derive(Debug)]
enum ChachaBuf{
    U([u32;16]),   
    C([u8;64])
}

impl ChachaBuf { 
    fn to_u(input: ChachaBuf) -> ChachaBuf { 
        let mut output = [0u32; 16];
        if let ChachaBuf::C(input) = input { 
            for i in 0..16 {
                output[i] = u32::from_le_bytes([
                    input[i * 4],
                    input[i * 4 + 1],
                    input[i * 4 + 2],
                    input[i * 4 + 3],
                ]);
            }
        } else { 
            panic!("Is not ChachaBuf::C");
        }
        ChachaBuf::U(output)
    }  
}

trait Round { 
    fn quarter_round(mut a: u32,mut b: u32,mut c: u32,mut d: u32) -> [u32;4] { 
        a += b;
        d ^= a;
        d = d.rotate_left(16);


        c += d;
        b ^= c;
        b = b.rotate_left(12);

        a += b;
        d ^= a;
        d = d.rotate_left(8);

        c += d;
        b ^= c;
        b = b.rotate_left(7);

        [a,b,c,d]
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
    let mut nonce = [0u8;12];
    let mut key = [0u8;32];
    let mut block_counter = [0u8;4];
    rng.fill(&mut nonce);
    rng.fill(&mut key);

    let key_init: ChachaBuf = ChachaBuf::C(
       [CONSTANTS.to_vec(),key.to_vec(),block_counter.to_vec(),nonce.to_vec()].concat().try_into().expect("key is not 64 bytes")
    ); 
    println!("Hello, world!{:?}",key_init);
}


#[cfg(test)] 
mod tests { 
    use super::*;

    #[test]
    fn one_test() { 
        assert_eq!("aa","aa")
    }
}