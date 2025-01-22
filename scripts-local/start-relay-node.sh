#!/bin/bash

# Check if a sample file argument is provided
SAMPLE_FILE="${1:-.env.testnet.sample}"

# Validate the existence of the specified sample file
if [ ! -f "$SAMPLE_FILE" ]; then
    echo "Sample file '$SAMPLE_FILE' not found. Please provide a valid sample file (e.g., '.env.mainnet.sample' or '.env.testnet.sample') as an argument."
    exit 1
fi

# Load environment variables from the .env file
if [ -f .env ]; then
    export $(grep -v '^#' .env | xargs)
else
    echo ".env file not found. Please create a .env file using the '$SAMPLE_FILE' template and try again."
    exit 1
fi

# Function to check if environment variables are set
check_env_vars() {
    local missing_vars=0
    local missing_keys=()

    while IFS= read -r line; do
        # Skip empty lines and comments
        [[ -z "$line" || "$line" == \#* ]] && continue

        # Extract the key
        key=$(echo "$line" | cut -d '=' -f 1)

        # Check if the key is set in the environment
        if [ -z "${!key}" ]; then
            echo "Environment variable $key is not set."
            missing_vars=$((missing_vars + 1))
            missing_keys+=("$key")
        fi
    done < "$SAMPLE_FILE"

    # If missing variables, display a summary
    if [ $missing_vars -gt 0 ]; then
        echo ""
        echo "Summary of missing environment variables:"
        for var in "${missing_keys[@]}"; do
            echo "- $var"
        done
    fi

    return $missing_vars
}

# Validate environment variables using the specified sample file
check_env_vars
if [ $? -ne 0 ]; then
    echo ""
    echo "One or more environment variables are missing. Please check the '$SAMPLE_FILE' file for required variables."
    exit 1
fi

echo "All required environment variables are set."


cargo run -p fuel-relay-node -- \
    --service-name "Fuel Relay Node" \
    --ip 0.0.0.0 \
    --port 4000 \
    --peering-port 30333 \
    --db-path tmp/fuel-relay-node-db  \
    --utxo-validation \
    --poa-instant false \
    --enable-p2p \
    --keypair $KEYPAIR \
    --snapshot ./fuel-nodes/fuel-node/chain-config/testnet \
    --enable-relayer \
    --relayer $RELAYER \
    --relayer-v2-listening-contracts $RELAYER_V2_LISTENING_CONTRACTS \
    --relayer-da-deploy-height $RELAYER_DA_DEPLOY_HEIGHT \
    --relayer-log-page-size $RELAYER_LOG_PAGE_SIZE \
    --reserved-nodes $RESERVED_NODES \
    --sync-header-batch-size $SYNC_HEADER_BATCH_SIZE \
    --sync-block-stream-buffer-size 50 \