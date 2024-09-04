use bevy::prelude::default;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use shared::config::create_shared_config;
use shared::sockets::*;

// Constructs the whole client and returns a ClientPlugins, which is basically a simple plugin
pub fn build_client() -> ClientPlugins {
    // Build the authentication key ideally this should be done by a third party app and just consumed
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: 0,
    };
    // Io is  short for input and output, it basically tells me the type of system I will utilize to send the data between the envolved parties. In online games the go to is udp
    let io = IoConfig {
        transport: ClientTransport::UdpSocket(CLIENT_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        auth: auth,
        config: NetcodeConfig::default(),
        io: io,
    };
    // Basically the centralization of all configs
    let client_config = ClientConfig {
        shared: create_shared_config(),
        net: net_config,
        ..default()
    };

    return ClientPlugins::new(client_config);
}
