pub trait NatsClient {
    fn is_connected(&self) -> bool {
        self.state() == async_nats::connection::State::Connected
    }

    fn state(&self) -> async_nats::connection::State {
        self.nats_client().connection_state()
    }

    fn nats_client(&self) -> async_nats::Client;
}
