#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimeType {
    Empty = 0,
    Block = 1,
    Tub = 2,
    Beehive = 3,
    Blinker = 4,
    Beacon = 5,
    Clock = 6,
    Toad = 7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prime<'a> {
    pub prime_type: PrimeType,
    pub exclusive_size: usize,
    pub occupied: &'a[&'a[u8]],
    pub prime: &'a[&'a[u8]],
}

pub const EMPTY: Prime<'static> = Prime {
    prime_type: PrimeType::Empty,
    exclusive_size: 1,
    occupied: &[&[0]],
    prime: &[&[0]],
};

pub const BLOCK: Prime<'static> = Prime {
    prime_type: PrimeType::Block,
    exclusive_size: 4,
    occupied: &[
        &[0, 0, 0, 0],
        &[0, 1, 1, 0],
        &[0, 1, 1, 0],
        &[0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0],
        &[0, 1, 1, 0],
        &[0, 1, 1, 0],
        &[0, 0, 0, 0]
    ],
};

pub const TUB: Prime<'static> = Prime {
    prime_type: PrimeType::Tub,
    exclusive_size: 5,
    occupied: &[
        &[0, 0, 0, 0, 0],
        &[0, 0, 1, 0, 0],
        &[0, 1, 1, 1, 0],
        &[0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0, 0],
        &[0, 0, 1, 0, 0],
        &[0, 1, 0, 1, 0],
        &[0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0]
    ],
};

pub const BEEHIVE: Prime<'static> = Prime {
    prime_type: PrimeType::Beehive,
    exclusive_size: 6,
    occupied: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 1, 1, 1, 1, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 1, 0, 0, 1, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0, 0]
    ],
};

pub const BLINKER: Prime<'static> = Prime {
    prime_type: PrimeType::Blinker,
    exclusive_size: 5,
    occupied: &[
        &[0, 0, 0, 0, 0],
        &[0, 0, 1, 0, 0],
        &[0, 1, 1, 1, 0],
        &[0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0],
        &[0, 1, 1, 1, 0],
        &[0, 0, 0, 0, 0],
        &[0, 0, 0, 0, 0],
    ]
};

pub const BEACON: Prime<'static> = Prime {
    prime_type: PrimeType::Beacon,
    exclusive_size: 6,
    occupied: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 1, 1, 0, 0, 0],
        &[0, 1, 1, 0, 0, 0],
        &[0, 0, 0, 1, 1, 0],
        &[0, 0, 0, 1, 1, 0],
        &[0, 0, 0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 1, 1, 0, 0, 0],
        &[0, 1, 0, 0, 0, 0],
        &[0, 0, 0, 0, 1, 0],
        &[0, 0, 0, 1, 1, 0],
        &[0, 0, 0, 0, 0, 0]
    ],
};

pub const CLOCK: Prime<'static> = Prime {
    prime_type: PrimeType::Clock,
    exclusive_size: 6,
    occupied: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 1, 0, 1, 1, 0],
        &[0, 1, 1, 0, 1, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 0, 0, 0],
        &[0, 0, 0, 1, 1, 0],
        &[0, 1, 1, 0, 0, 0],
        &[0, 0, 0, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0]
    ]
};

pub const TOAD: Prime<'static> = Prime {
    prime_type: PrimeType::Toad,
    exclusive_size: 6,
    occupied: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 1, 1, 1, 0, 0],
        &[0, 0, 1, 1, 1, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0]
    ],
    prime: &[
        &[0, 0, 0, 0, 0, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 1, 0, 0, 0, 0],
        &[0, 0, 0, 0, 1, 0],
        &[0, 0, 1, 1, 0, 0],
        &[0, 0, 0, 0, 0, 0]
    ]
};

