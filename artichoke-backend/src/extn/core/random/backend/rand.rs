use rand::{self, Rng, SeedableRng};
use rand_pcg::Pcg64;
use std::fmt;

use crate::extn::core::random::backend::{InternalState, RandType};
use crate::extn::prelude::*;

#[must_use]
pub fn new(seed: Option<u64>) -> Box<dyn RandType> {
    Box::new(Rand::<Pcg64>::new(seed))
}

#[derive(Debug, Clone)]
pub struct Rand<T> {
    rng: T,
    seed: u64,
}

impl<T> Rand<T>
where
    T: SeedableRng,
{
    pub fn new(seed: Option<u64>) -> Self {
        let seed = seed.unwrap_or_else(rand::random);
        let rng = T::seed_from_u64(seed);
        Self { rng, seed }
    }
}

impl<T> Rand<T> {
    #[inline]
    pub fn seed(&self) -> u64 {
        self.seed
    }
}

impl<T> Rand<T>
where
    T: 'static,
{
    pub fn internal_state(&self) -> InternalState {
        // This is not quite right. It needs to take into account bytes read
        // from the PRNG.
        InternalState::Rand { seed: self.seed }
    }
}

impl<T> Rand<T>
where
    T: Rng,
{
    #[inline]
    pub fn bytes(&mut self, buf: &mut [u8]) {
        // TODO: use non-panicking ``Rng::try_fill_bytes`, which requires a
        // custom error type and returning `Result<(), Exception>`.
        self.rng.fill_bytes(buf);
    }

    #[inline]
    pub fn rand_int(&mut self, max: Int) -> Int {
        self.rng.gen_range(0, max)
    }

    #[inline]
    pub fn rand_float(&mut self, max: Option<Float>) -> Float {
        let max = max.unwrap_or(1.0);
        self.rng.gen_range(0.0, max)
    }
}

impl<T> RandType for Rand<T>
where
    T: 'static + Rng + fmt::Debug,
{
    fn as_debug(&self) -> &dyn fmt::Debug {
        self
    }

    fn bytes(&mut self, interp: &mut Artichoke, buf: &mut [u8]) {
        let _ = interp;
        self.bytes(buf);
    }

    fn seed(&self, interp: &Artichoke) -> u64 {
        let _ = interp;
        self.seed()
    }

    fn internal_state(&self, interp: &Artichoke) -> InternalState {
        let _ = interp;
        self.internal_state()
    }

    fn rand_int(&mut self, interp: &mut Artichoke, max: Int) -> Int {
        let _ = interp;
        self.rand_int(max)
    }

    fn rand_float(&mut self, interp: &mut Artichoke, max: Option<Float>) -> Float {
        let _ = interp;
        self.rand_float(max)
    }
}
