use std::{io::Read,collections::HashMap};

use anyhow::{Ok, Result};
use base64::{prelude::BASE64_URL_SAFE_NO_PAD, Engine};

use crate::{get_reader, TextSignFormat};

trait TextSigner {
    // fn sign(&self , data: &str) ->Result<Vec<u8>,()>;
    //reader 具有更高的灵活性
    fn sign(&self, reader:&mut dyn Read) -> Result<Vec<u8>>;
}

trait TextVerifier {
    //改成impl 效率会更高，但是代码的体积会更高
    fn verify(&self,reader:&mut dyn Read, sig:&[u8]) -> Result<bool>;
}


struct Black3 {
    key:[u8;32],
}

// struct Ed25519Signer {
//     key:[u8;32],
// }

// struct Ed25519Verifier {
//     key:[u8;32],
// }

pub fn process_sign(input:&str, key: &str , format: TextSignFormat) ->Result<(),anyhow::Error> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Black3 => todo!(),
        TextSignFormat::Ed25519 => todo!(),
    };
    // let signed = BASE64_URL_SAFE_NO_PAD.encode(input:&signed);
    // println!("{}",signed);
    Ok(())
}

impl TextSigner for Black3 {
    fn sign(&self, reader:&mut dyn Read) -> Result<Vec<u8>> {

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        Ok(blake3::hash(&buf).as_bytes().to_vec())
    }
}

impl TextVerifier for Black3 {
    fn verify(&self,reader:&mut dyn Read, sig:&[u8]) -> Result<bool> {

        let mut buf = Vec::new();
        reader.read_to_end(&mut buf);
        let ret = blake3::keyed_hash(&self.key,&buf);
        Ok(ret.as_bytes() == sig)
    }
}
