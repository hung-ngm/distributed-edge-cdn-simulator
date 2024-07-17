# Distributed Edge CDN Simulator

## Overview

This project simulates a Distributed Edge Content Delivery Network (CDN) using Rust and Tokio. It models the behavior of content distribution and request handling in a CDN, providing insights into scheduling algorithms, network latency, and system performance.

## High-Level Design

The simulator consists of several key components:

1. **Content**: Represents the data being distributed and served by the CDN.

2. **EdgeNode**: Simulates individual servers in the CDN, each with a specific capacity and stored content.

3. **CDN**: The main structure that manages edge nodes, content distribution, and request handling.

4. **Request**: Represents user requests for specific content.

5. **NetworkSimulator**: Simulates network conditions, including latency and jitter.

### Key Features

- Asynchronous content distribution and request handling using Tokio
- Basic scheduling algorithm for optimal content placement
- Network latency simulation for realistic behavior
- Concurrent request processing

## System Components

### Content Distribution

The CDN uses an optimized distribution algorithm to place content on edge nodes. It considers factors such as node capacity and current load to determine the best placement.

### Request Handling

When a request is received, the CDN searches for the content across its edge nodes. If found, it simulates serving the content, accounting for network latency.

### Network Simulation

A `NetworkSimulator` adds realistic delay to operations, simulating the latency and jitter of a real network environment.

### Concurrency

The system leverages Tokio to handle multiple requests concurrently, demonstrating the CDN's ability to serve many users simultaneously.

## Future Enhancements

- Implement more sophisticated scheduling algorithms
- Add failure scenarios and recovery mechanisms
- Implement content popularity simulation and advanced caching strategies
- Add detailed logging and metrics collection
- Create a visualization of the CDN's state and performance

## Getting Started


## Testing

