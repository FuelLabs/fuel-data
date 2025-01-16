.PHONY: start-live-pub-nats stop-live-pub-nats restart-live-pub-nats 


setup:
	./scripts/setup.sh

# ------------------------------------------------------------
#  LIVE PUBLISHER
# ------------------------------------------------------------
start-live-pub:
	./scripts/start-live-pub.sh
	
start-live-pub-nats:
	docker-compose -f docker/docker-compose.yml up -d live-publisher-nats

stop-live-pub-nats:
	docker-compose -f docker/docker-compose.yml stop live-publisher-nats
	 
restart-live-pub-nats: start-live-pub-nats stop-live-pub-nats


