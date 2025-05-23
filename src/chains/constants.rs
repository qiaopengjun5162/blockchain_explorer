#[allow(dead_code)]
pub mod alchemy {
    pub mod ethereum {
        pub const MAINNET: &str = "eth-mainnet.g.alchemy.com/v2";
        pub const SEPOLIA: &str = "eth-sepolia.g.alchemy.com/v2";
        pub const HOLESKY: &str = "eth-holesky.g.alchemy.com/v2";
        pub const HOODI: &str = "eth-hoodi.g.alchemy.com/v2";
                               
    }

    pub mod polygon {
        pub const MAINNET: &str = "polygon-mainnet.g.alchemy.com/v2";
        pub const AMOY: &str = "polygon-amoy.g.alchemy.com/v2";
    }

    pub mod zksync {
        pub const MAINNET: &str = "zksync-mainnet.g.alchemy.com/v2";
        pub const SEPOLIA: &str = "zksync-sepolia.g.alchemy.com/v2";
    }
}