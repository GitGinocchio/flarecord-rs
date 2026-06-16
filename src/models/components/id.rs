use std::sync::LazyLock;
use rand::{random, random_range};

const CHARSET: &[u8; 62] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub (crate) static ID_GEN: LazyLock<ComponentIdGenerator> = LazyLock::new(ComponentIdGenerator::new);

pub (crate) struct ComponentIdGenerator;

impl ComponentIdGenerator {
    const fn new() -> Self {
        Self
    }

    pub fn next(&self) -> String {
        (0..4)
            .map(|_| {
                let idx = random_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    pub fn next_i32(&self) -> i32 {
        random::<u8>() as i32
    }
}