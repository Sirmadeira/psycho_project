use bevy::prelude::*;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use lightyear::server::config::NetcodeConfig;
use lightyear::server::plugin::ServerPlugins;
use shared::config::*;
use shared::sockets::*;

/// Here we create the lightyear [`ServerPlugins`]
pub fn build_server_plugin() -> ServerPlugins {
    // The IoConfig will specify the transport to use.
    let io = IoConfig {
        // the address specified here is the server_address, because we open a UDP socket on the server
        transport: ServerTransport::UdpSocket(SERVER_ADDR),
        ..default()
    };
    // The NetConfig specifies how we establish a connection with the server.
    // We can use either Steam (in which case we will use steam sockets and there is no need to specify
    // our own io) or Netcode (in which case we need to specify our own io).
    let net_config = NetConfig::Netcode {
        io,
        config: NetcodeConfig::default(),
    };
    let config = ServerConfig {
        // part of the config needs to be shared between the client and server
        shared: create_shared_config(),
        // we can specify multiple net configs here, and the server will listen on all of them
        // at the same time. Here we will only use one
        net: vec![net_config],
        replication: ReplicationConfig {
            // we will send updates to the clients every 100ms
            send_interval: SERVER_REPLICATION_INTERVAL,
            ..default()
        },
        ..default()
    };
    info!("Server is built");
    ServerPlugins::new(config)
}
