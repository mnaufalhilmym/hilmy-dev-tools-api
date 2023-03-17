use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2,
};

pub fn new<'a>(secret: &'a [u8]) -> (Argon2, SaltString) {
    (new_argon2(secret), SaltString::generate(&mut OsRng))
}

pub fn new_argon2<'a>(secret: &'a [u8]) -> Argon2 {
    argon2::Argon2::new_with_secret(
        secret,
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::default(),
    )
    .unwrap()
}
