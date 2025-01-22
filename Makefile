.PHONY: start-relay-node stop-relay-node restart-relay-node start-archive-node stop-archive-node restart-archive-node


setup:
	./scripts-local/setup.sh

# ------------------------------------------------------------
#  Archive Node
# ------------------------------------------------------------
start-archive-node:
	docker-compose -f fuel-data-cluster/docker/docker-compose.yml up -d archive-nats
	./scripts-local/start-archive-node.sh
	
# TODO: Complete this step by killing binary 
stop-archive-node:
	docker-compose -f fuel-data-cluster/docker/docker-compose.yml stop archive-nats
	 
restart-archive-node: 
	stop-archive-node 
	rm -rf ./tmp/fuel-archive-node-db
	start-archive-node

# ------------------------------------------------------------
#  Relay Node
# ------------------------------------------------------------
start-relay-node:
	docker-compose -f fuel-data-cluster/docker/docker-compose.yml up -d relay-nats
	./scripts-local/start-relay-node.sh
	
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

