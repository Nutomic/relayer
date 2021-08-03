pub mod substrate {
    #![allow(dead_code)]
    macro_rules! define_chain {
        ($name:ident => $endpoint:expr) => {
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            pub struct $name;
            impl $name {
                pub const fn endpoint() -> &'static str { $endpoint }
            }
        };
        ($($name:ident => $endpoint:expr),+) => {
            $(define_chain!($name => $endpoint);)+
        }
    }

    define_chain! {
        Edgeware => "wss://mainnet1.edgewa.re",
        Beresheet => "wss://beresheet1.edgewa.re",
        Webb => "ws://127.0.0.1:9944"
    }
}

pub mod evm {
    use std::collections::HashMap;
    use std::str::FromStr;

    use webb::evm::ethereum_types::Address;
    /// All Supported Chains by Webb Realyer.
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub enum ChainName {
        Edgeware,
        Webb,
        Ganache,
        Beresheet,
        Harmony,
    }

    pub trait EvmChain {
        fn name() -> ChainName;
        fn endpoint() -> &'static str;
        fn ws_endpoint() -> &'static str;
        fn chain_id() -> u32;
        fn contracts() -> HashMap<Address, u128>;
    }

    macro_rules! define_chain {
        ($name:ident => {
            endpoint: $endpoint:expr,
            ws_endpoint: $ws_endpoint:expr,
            chain_id: $chain_id:expr,
            contracts: [
                $({
                    size: $size:expr,
                    address: $address:expr,
                }),*
            ],
        }) => {
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            pub struct $name;
            impl EvmChain for $name {
                fn name() -> ChainName { ChainName::$name }

                fn endpoint() -> &'static str { $endpoint }

                fn ws_endpoint() -> &'static str { $ws_endpoint }

                fn chain_id() -> u32 { $chain_id }

                fn contracts() -> HashMap<Address, u128> {
                    #[allow(unused_mut)]
                    let mut map = HashMap::new();
                    $(
                        let address = Address::from_str($address).unwrap();
                        map.insert(address, $size);
                    )*
                    map
                }
            }
       };
    }

    define_chain! {
        Edgeware => {
            endpoint: "https://mainnet1.edgewa.re/evm",
            ws_endpoint: "wss://mainnet1.edgewa.re/evm",
            chain_id: 2021,
            contracts: [],
        }
    }

    define_chain! {
        Ganache => {
            endpoint: "http://localhost:1998",
            ws_endpoint: "ws://localhost:8545",
            chain_id: 1337,
            contracts: [
                {
                    size: 1,
                    address: "0xf759e19b1142079b1963e1e323b07e4ac67ab899",
                }
            ],
        }
    }

    define_chain! {
        Beresheet => {
            endpoint: "https://beresheet.edgewa.re/evm",
            ws_endpoint: "wss://beresheet.edgewa.re/evm",
            chain_id: 2022,
            contracts: [
                {
                    size: 10,
                    address: "0xf0EA8Fa17daCF79434d10C51941D8Fc24515AbE3",
                },
                {
                    size: 100,
                    address: "0xc0d863EE313636F067dCF89e6ea904AD5f8DEC65",
                },
                {
                    size: 1_000,
                    address: "0xc7c6152214d0Db4e161Fa67fB62811Be7326834A",
                },
                {
                    size: 10_000,
                    address: "0xf0290d80880E3c59512e454E303FcD48f431acA3",
                }
            ],
        }
    }

    define_chain! {
        Harmony => {
            endpoint: "https://api.s1.b.hmny.io",
            ws_endpoint: "wss://ws.s1.b.hmny.io",
            chain_id: 1666700001,
            contracts: [
                {
                    size: 100,
                    address: "0x7cd1F52e5EEdf753e99D945276a725CE533AaD1a",
                },
                {
                    size: 1_000,
                    address: "0xD7f9BB9957100310aD397D2bA31771D939BD4731",
                },
                {
                    size: 10_000,
                    address: "0xeE2eB8F142e48e5D1bDD34e0924Ed3B4aa0d4222",
                },
                {
                    size: 100_000,
                    address: "0x7cd173094eF78FFAeDee4e14576A73a79aA716ac",
                }
            ],
        }
    }

    define_chain! {
        Webb => {
            endpoint: "",
            ws_endpoint: "",
            chain_id: 0,
            contracts: [],
        }
    }
}
