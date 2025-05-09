use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};

use lazy_static::lazy_static;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};
use spacetimedb::SpacetimeType;
use crate::board::Player;
use crate::games::go::{FlatTile, State, GO_MAX_AREA};
use crate::util::tiny::consistent_rng;

type Inner = u128;

#[derive(SpacetimeType, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Zobrist {
    pub v: Inner,
}

pub struct HashData {
    color_tile: [Vec<Zobrist>; 2],
    color_turn: [Zobrist; 2],
    pass_state: [Zobrist; 3],
}

impl Zobrist {
    pub fn for_color_tile(color: Player, tile: FlatTile) -> Zobrist {
        HASH_DATA.color_tile[color.index() as usize][tile.index() as usize]
    }

    pub fn for_color_turn(color: Player) -> Zobrist {
        HASH_DATA.color_turn[color.index() as usize]
    }

    pub fn for_pass_state(state: State) -> Zobrist {
        // don't include outcome, that is implicit from the other tiles anyway
        let state_index = match state {
            State::Normal => 0,
            State::Passed => 1,
            State::Done(_) => 2,
        };
        HASH_DATA.pass_state[state_index]
    }
}

// TODO generate this at compile-time?
lazy_static! {
    static ref HASH_DATA: HashData = HashData::new();
}

impl HashData {
    #[allow(clippy::new_without_default)]
    #[inline(never)]
    pub fn new() -> HashData {
        let mut rng = consistent_rng();
        let vec_len = GO_MAX_AREA as usize;
        HashData {
            color_tile: [gen_vec(vec_len, &mut rng), gen_vec(vec_len, &mut rng)],
            color_turn: gen_array(&mut rng),
            pass_state: gen_array(&mut rng),
        }
    }
}

impl Debug for HashData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HashData").finish_non_exhaustive()
    }
}

impl Distribution<Zobrist> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Zobrist {
        Zobrist{v:rng.gen()}
    }
}

fn gen_array<const N: usize>(rng: &mut impl Rng) -> [Zobrist; N] {
    let mut array = [Zobrist::default(); N];
    for x in &mut array {
        *x = rng.gen();
    }
    array
}

fn gen_vec(len: usize, rng: &mut impl Rng) -> Vec<Zobrist> {
    Standard.sample_iter(rng).take(len).collect()
}

impl Debug for Zobrist {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // print hex, full-width with leading 0x
        write!(
            f,
            "Zobrist({:#0width$x})",
            self.v,
            width = (Inner::BITS / 8 + 2) as usize
        )
    }
}

impl std::ops::BitXor for Zobrist {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Zobrist{v:self.v ^ rhs.v}
    }
}

impl std::ops::BitXorAssign for Zobrist {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.v ^= rhs.v;
    }
}

impl nohash_hasher::IsEnabled for Zobrist {}

impl Hash for Zobrist {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64((self.v as u64) ^ ((self.v >> 64) as u64));
    }
}
