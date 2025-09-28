#!/bin/sh

# Run database migrations
echo "Running database migrations..."
diesel migration run

# Start the main application
echo "Starting application..."
./contacts-api
