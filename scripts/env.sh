#!/bin/bash

# Function to load the .env file
load_env_file() {
    local network="$1"
    local env_file=".env.$1"

    if [ -f $env_file ]; then
        export $(grep -v '^#' $env_file | xargs)
    else
        echo "'$env_file' file not found. Please create a .env file using the appropriate template and try again."
        exit 1
    fi
}

# Function to check if environment variables are set
check_env_vars() {
    local env_sample_file="$1"
    local missing_vars=0
    local missing_keys=()

    if [ ! -f "$env_sample_file" ]; then
        echo "Sample file '$env_sample_file' not found."
        return 1
    fi

    while IFS= read -r line; do
        # Skip empty lines and comments
        [[ -z "$line" || "$line" == \#* ]] && continue

        # Extract the key
        key=$(echo "$line" | cut -d '=' -f 1)

        # Check if the key is set in the environment
        if [ -z "${!key}" ]; then
            echo "Environment variable $key is not set."
            missing_keys+=("$key")
            missing_vars=$((missing_vars + 1))
        fi
    done < "$env_sample_file"

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
