use bcrypt::{BcryptResult, DEFAULT_COST};
use rand::distributions::{Alphanumeric, DistString};

fn salt() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

fn hash_with_salt<P: AsRef<[u8]>>(pwd: P, salt: [u8; 16]) -> String {
    bcrypt::hash_with_salt(pwd, DEFAULT_COST, salt).unwrap().to_string()
}

fn verify<P: AsRef<[u8]>>(pwd: P, pwd_in_db: &str) -> BcryptResult<bool> {
    bcrypt::verify(pwd, pwd_in_db)
}

#[cfg(test)]
mod test{
    use super::*;

    const PWD_IN_DB: &str = "$2y$12$PUXUPEyvWiXgaijATl/EQOP0p5qyaYgeGC4KTTRbGR4bPi3T2rc6K";
    const SALT_IN_DB: &str = "EfVDm1bFbrIBVpFI";

    #[test]
    fn test_encode() {
        let user_input_pwd = "abcdefghijklmn";
        let mut bytes: [u8;16] = [0; 16];
        bytes.copy_from_slice(SALT_IN_DB.as_bytes());
        let encode_pwd = hash_with_salt(user_input_pwd, bytes);
        assert_eq!(encode_pwd, PWD_IN_DB);
    }

    #[test]
    fn test_decode() {
        let user_input_pwd = "abcdefghijklmn";
        assert!(verify(user_input_pwd, PWD_IN_DB).is_ok());
    }
}