use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use bitcoin::{blockdata::block::Header, consensus::Params, constants, p2p::Magic, Block};

use crate::{Error, Result};

/// Bitcoin network types.
#[derive(Debug, Copy, Clone)]
pub enum Network {
    /// Bitcoin Mainnet.
    Mainnet,
    /// Bitcoin Testnet.
    Testnet,
    /// Bitcoin regression test net.
    Regtest,
    /// Bitcoin signet.
    Signet,
}

impl Default for Network {
    fn default() -> Self {
        Self::Mainnet
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Network::Mainnet => write!(f, "mainnet"),
            Network::Testnet => write!(f, "testnet"),
            Network::Regtest => write!(f, "regtest"),
            Network::Signet => write!(f, "signet"),
        }
    }
}

impl FromStr for Network {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "mainnet" | "bitcoin" => Ok(Self::Mainnet),
            "testnet" => Ok(Self::Testnet),
            "regtest" => Ok(Self::Regtest),
            "signet" => Ok(Self::Signet),
            _ => Err(Error::InvalidNetwork(s.to_string())),
        }
    }
}

impl From<Network> for bitcoin::Network {
    fn from(network: Network) -> Self {
        match network {
            Network::Mainnet => Self::Bitcoin,
            Network::Testnet => Self::Testnet,
            Network::Regtest => Self::Regtest,
            Network::Signet => Self::Signet,
        }
    }
}

impl TryFrom<bitcoin::Network> for Network {
    type Error = Error;

    fn try_from(network: bitcoin::Network) -> Result<Self> {
        match network {
            bitcoin::Network::Bitcoin => Ok(Self::Mainnet),
            bitcoin::Network::Testnet => Ok(Self::Testnet),
            bitcoin::Network::Regtest => Ok(Self::Regtest),
            bitcoin::Network::Signet => Ok(Self::Signet),
            _ => Err(Error::InvalidNetwork(network.to_string())),
        }
    }
}

impl Network {
    /// Get the genesis block.
    pub fn genesis_block(&self) -> Block {
        constants::genesis_block((*self).into())
    }

    /// Get the genesis block header.
    pub fn genesis(&self) -> Header {
        self.genesis_block().header
    }

    /// Get the consensus parameters for this network.
    pub fn params(&self) -> Params {
        Params::new((*self).into())
    }

    /// Get the network magic number for this network.
    pub fn magic(&self) -> Magic {
        bitcoin::Network::from(*self).magic()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{Network, Result};

    #[test]
    fn network_from_str_works() -> Result<()> {
        assert_eq!("mainnet", format!("{}", Network::from_str("mainnet")?));
        assert_eq!("mainnet", format!("{}", Network::from_str("bitcoin")?));
        assert_eq!("testnet", format!("{}", Network::from_str("testnet")?));
        assert_eq!("regtest", format!("{}", Network::from_str("regtest")?));
        assert_eq!("signet", format!("{}", Network::from_str("signet")?));

        assert_eq!(
            format!("{}", Network::from_str("invalid").err().unwrap()),
            "Invalid network: invalid"
        );

        Ok(())
    }

    #[test]
    fn network_try_from_works() -> Result<()> {
        assert_eq!(
            "mainnet",
            format!("{}", Network::try_from(bitcoin::Network::Bitcoin)?)
        );
        assert_eq!(
            "testnet",
            format!("{}", Network::try_from(bitcoin::Network::Testnet)?)
        );
        assert_eq!(
            "regtest",
            format!("{}", Network::try_from(bitcoin::Network::Regtest)?)
        );
        assert_eq!(
            "signet",
            format!("{}", Network::try_from(bitcoin::Network::Signet)?)
        );

        Ok(())
    }
}
