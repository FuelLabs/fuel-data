#!/bin/bash

# Load environment variables from .env file
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo ".env file not found. Please create a .env file using the '.env.sample' template and try again."
    exit 1
fi


cargo run -p fuel-data-live-publisher -- \
    --service-name "Fuel Node for Live Publisher" \
    --ip 0.0.0.0 \
    --port 4000 \
    --peering-port 30333 \
    --db-path tmp/fuel-node-for-live-publisher-db  \
    --utxo-validation \
    --poa-instant false \
    --enable-p2p \
    --keypair $KEYPAIR \
    --snapshot ./crates/fuel-node/chain-config/testnet \
    --enable-relayer \
    --relayer $RELAYER \
    --relayer-v2-listening-contracts $RELAYER_V2_LISTENING_CONTRACTS \
    --relayer-da-deploy-height $RELAYER_DA_DEPLOY_HEIGHT \
    --relayer-log-page-size $RELAYER_LOG_PAGE_SIZE \
    --reserved-nodes $RESERVED_NODES \
    --sync-header-batch-size $SYNC_HEADER_BATCH_SIZE \
    --sync-block-stream-buffer-size 50 \