.PHONY: start-relay-node stop-relay-node restart-relay-node


setup:
	./scripts/setup.sh

# ------------------------------------------------------------
#  LIVE PUBLISHER
# ------------------------------------------------------------
start-relay-node:
	docker-compose -f docker/docker-compose.yml up -d relay-node-nats
	./scripts/start-live-pub.sh
	
# TODO: Complete this step by killing binary 
stop-relay-node:
	docker-compose -f docker/docker-compose.yml stop live-publisher-nats
	 
restart-relay-node: start-relay-node stop-relay-node


