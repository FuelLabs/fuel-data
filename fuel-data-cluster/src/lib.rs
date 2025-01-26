// TODO: Return URI types here

pub mod where_is {
    pub fn relay_nats() -> String {
        dotenvy::var("RELAY_NATS_URL").expect("RELAY_NATS_URL must be set")
    }
}

pub mod where_are {
    use k8s_openapi::api::core::v1::Pod;
    use kube::api::ListParams;
    use kube::{Api, Client};

    pub async fn latest_archive_nats() -> String {
        archive_nats()
            .await
            .last()
            .expect("There must be at least one archive nats")
            .clone()
    }

    pub async fn archive_nats() -> Vec<String> {
        let default = dotenvy::var("ARCHIVE_NATS_URL").expect("ARCHIVE_NATS_URL must be set");

        let mut all_in_cluster = {
            if let Ok(client) = Client::try_default().await {
                let pods: Api<Pod> = Api::namespaced(client, "fuel-data");
                let list_params = ListParams::default().labels("app=archive-nats");
                let pod_list = pods
                    .list(&list_params)
                    .await
                    .expect("Kube should return all pods instances");

                pod_list
                    .items
                    .iter()
                    .filter_map(|pod| {
                        let ip = pod.status.as_ref()?.pod_ip.as_ref()?;
                        let timestamp = pod.metadata.creation_timestamp.as_ref().map(|ts| ts.0);
                        timestamp.map(|ts| (format!("{}:4223", ip), ts))
                    })
                    .collect::<Vec<_>>()
            } else {
                tracing::warn!(
                    "Client not found, using default Archive NATS URL: {}",
                    &default,
                );
                vec![]
            }
        };

        // Sort from oldest to newest
        all_in_cluster.sort_by(|a, b| a.1.cmp(&b.1));

        if all_in_cluster.is_empty() {
            vec![default]
        } else {
            all_in_cluster
                .into_iter()
                .map(|(nats_url, _)| nats_url)
                .collect()
        }
    }
}
