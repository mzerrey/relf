#!/bin/bash

# Colors for better readability
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to display usage
show_usage() {
  echo -e "${BLUE}Relf Docker Helper Script${NC}"
  echo ""
  echo "Usage: $0 [command]"
  echo ""
  echo "Commands:"
  echo "  build       Build the Docker image"
  echo "  run         Build and run the container"
  echo ""
  echo "  stop        Stop and remove running containers"
  echo ""
  echo "  logs        Show logs from the running container"
  echo ""
  echo "  clean       Clean up Docker images and containers"
  echo "  help        Show this help message"
  echo ""
}

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
  echo "Error: Docker is not installed or not in PATH"
  exit 1
fi

IMAGE_NAME="relf"
CONTAINER_NAME="relf-container"

# Process commands
case "$1" in
  build)
    echo -e "${GREEN}Building Docker image...${NC}"
    docker build -t $IMAGE_NAME .
    echo -e "${GREEN}Image built successfully!${NC}"
    ;;
    
  run)
    echo -e "${GREEN}Building and running Relf application...${NC}"
    
    # Stop existing container if running
    if docker ps -q -f name=$CONTAINER_NAME; then
      echo -e "${BLUE}Stopping existing container...${NC}"
      docker stop $CONTAINER_NAME
      docker rm $CONTAINER_NAME
    fi
    
    # Build image
    docker build -t $IMAGE_NAME .
    
    # Run container
    docker run -d --name $CONTAINER_NAME -p 5000:5000 $IMAGE_NAME
    
    echo -e "${GREEN}Container is running!${NC}"
    echo -e "Access the application at: ${BLUE}http://localhost:5000${NC}"
    echo -e "Available pages:"
    echo -e "  - ${BLUE}http://localhost:5000/relf${NC} - Main application"
    echo -e "To view logs, run: ${BLUE}./docker-run.sh logs${NC}"
    ;;
    
    
  stop)
    echo -e "${GREEN}Stopping container...${NC}"
    docker stop $CONTAINER_NAME 2>/dev/null || echo "Container was not running"
    docker rm $CONTAINER_NAME 2>/dev/null || echo "Container was not found"
    echo -e "${GREEN}Container stopped${NC}"
    ;;
    
    
  logs)
    echo -e "${GREEN}Showing logs for Relf container:${NC}"
    docker logs -f $CONTAINER_NAME
    ;;
    
    
  clean)
    echo -e "${GREEN}Cleaning up Docker images and containers...${NC}"
    docker stop $CONTAINER_NAME 2>/dev/null || true
    docker rm $CONTAINER_NAME 2>/dev/null || true
    docker rmi $IMAGE_NAME 2>/dev/null || true
    echo -e "${GREEN}Cleanup completed${NC}"
    ;;
    
  help|*)
    show_usage
    ;;
esac
