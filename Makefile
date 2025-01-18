.PHONY: start-relay-node stop-relay-node restart-relay-node


setup:
	./scripts/setup.sh

# ------------------------------------------------------------
#  Relay Node
# ------------------------------------------------------------
start-relay-node:
	docker-compose -f fuel-data-cluster/docker/docker-compose.yml up -d relay-nats
	./scripts/start-relay-node.sh
	
# TODO: Complete this step by killing binary 
stop-relay-node:
	docker-compose -f fuel-data-cluster/docker/docker-compose.yml stop relay-nats
	 
restart-relay-node: 
	stop-relay-node 
	rm -rf ./tmp/fuel-relay-node-db
	start-relay-node

start-grpc-edge:
	cargo run -p fuel-data-grpc-edge

run-examples:
	cargo run -p fuel-data-rs

