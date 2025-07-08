use sha1::{Digest, Sha1};

pub fn get_player_digest(name: &str) -> String {
    let mut hash: [u8; 20] = Sha1::new().chain_update(name).finalize().into();

    let negative = (hash[0] & 0x80) != 0;

    if negative {
        let mut carry = true;
        for byte in hash.iter_mut().rev() {
            *byte = !*byte;
            if carry {
                *byte = byte.wrapping_add(1);
                carry = *byte == 0;
            }
        }
    }

    // Encode to hex
    let mut hex = String::with_capacity(41);
    if negative {
        hex.push('-');
    }
    let mut started = false;
    for byte in hash.iter() {
        for nibble in [byte >> 4, byte & 0x0F] {
            if !started {
                if nibble == 0 {
                    continue;
                }
                started = true;
            }
            hex.push(std::char::from_digit(nibble as u32, 16).unwrap());
        }
    }

    if hex == "-" {
        "0".to_string()
    } else {
        hex
    }
}
