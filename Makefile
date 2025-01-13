.PHONY: start-nats stop-nats restart-nats start-s3 stop-s3 restart-s3 start-nats-and-s3 stop-nats-and-s3 restart-nats-and-s3

# Start only the NATS service
start-nats:
	docker-compose -f docker/docker-compose.yml up -d nats

# Stop only the NATS service
stop-nats:
	docker-compose -f docker/docker-compose.yml stop nats

# Restart only the NATS service
restart-nats: stop-nats start-nats

# Start only the S3 service
start-s3:
	docker-compose -f docker/docker-compose.yml up -d s3

# Stop only the S3 service
stop-s3:
	docker-compose -f docker/docker-compose.yml stop s3

# Restart only the S3 service
restart-s3: stop-s3 start-s3

# Start both NATS and S3 services
start-nats-and-s3:
	docker-compose -f docker/docker-compose.yml up -d nats s3

# Stop both NATS and S3 services
stop-nats-and-s3:
	docker-compose -f docker/docker-compose.yml stop nats s3

# Restart both NATS and S3 services
restart-nats-and-s3: stop-nats-and-s3 start-nats-and-s3
