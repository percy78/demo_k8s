#!/bin/bash
set -e

# Build and deploy script for the Kubernetes operator

echo "Building Rust Kubernetes Operator..."

# Build the Docker image
echo "Building Docker image..."
docker build -t k8s-operator:latest .

echo "Docker image built successfully!"

# Deploy to Kubernetes
echo "Deploying to Kubernetes default namespace..."

# Apply CRD first
kubectl apply -f k8s/crd.yaml

# Apply RBAC
kubectl apply -f k8s/rbac.yaml

# Apply the deployment
kubectl apply -f k8s/deployment.yaml

echo "Operator deployed successfully!"

# Wait for the operator to be ready
echo "Waiting for operator to be ready..."
kubectl wait --for=condition=available --timeout=300s deployment/k8s-operator -n default

echo "Operator is ready!"

# Show deployment status
kubectl get pods -n default -l app=k8s-operator

echo ""
echo "To test the operator, apply the example AppManager:"
echo "kubectl apply -f k8s/example-appmanager.yaml"
echo ""
echo "To check the status:"
echo "kubectl get appmanagers -n default"
echo "kubectl describe appmanager example-app -n default"