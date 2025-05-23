use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ConnectionType {
    #[default]
    Ws,
    Https,
    Ipc,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum ProviderService {
    #[default]
    Alchemy,
    Infura,
    Other,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ChainProvider {
    pub name: String,
    pub connection_type: ConnectionType,
    pub is_mainnet: bool,
    pub rpc_url: String,
    pub provider_service: ProviderService, // Alchemy, Infura, or other
    pub api_key: String,
}

impl ChainProvider {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_connection_type(mut self, connection_type: ConnectionType) -> Self {
        self.connection_type = connection_type;
        self
    }

    pub fn with_is_mainnet(mut self, is_mainnet: bool) -> Self {
        self.is_mainnet = is_mainnet;
        self
    }

    pub fn with_rpc_url(mut self, rpc_url: String) -> Self {
        self.rpc_url = rpc_url;
        self
    }

    pub fn with_provider_service(mut self, provider_service: ProviderService) -> Self {
        self.provider_service = provider_service;
        self
    }

    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }
}

impl Display for ChainProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let conn_type = match self.connection_type {
            ConnectionType::Ws => "wss",
            ConnectionType::Https => "https",
            ConnectionType::Ipc => "https",
        };

   
        write!(
            f,
            "{}://{}/{}", conn_type, self.rpc_url, self.api_key
        )
    }
}
