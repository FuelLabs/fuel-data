pub mod where_is {
    pub fn relay_nats() -> String {
        dotenvy::var("RELAY_NATS_URL").expect("RELAY_NATS_URL must be set for admin role")
    }
}

pub mod where_are {
    pub async fn archive_nats() -> Vec<String> {
        let nats_url =
            dotenvy::var("ARCHIVE_NATS_URL").expect("ARCHIVE_NATS_URL must be set for admin role");

        // TODO: Get the rest of the nats urls from kube
        vec![nats_url]
    }
}
