# DisasterMesh Comprehensive Testing Plan

## Testing Overview

This document outlines comprehensive testing strategies for the DisasterMesh project, covering unit tests, integration tests, performance tests, security tests, and real-world scenario validation.

## Testing Framework Setup

### Core Testing Dependencies
```toml
[dev-dependencies]
tokio-test = "0.4"
quickcheck = "1.0"
quickcheck_macros = "1.0"
criterion = "0.5"
mockall = "0.11"
tempfile = "3.0"
pretty_assertions = "1.0"
proptest = "1.0"
wiremock = "0.5"
serial_test = "2.0"
# Fuzzing & concurrency testing
cargo-fuzz = "0.13"
honggfuzz = "0.5"
loom = "0.5"
```

### Test Organization Structure
```
tests/
├── unit/                  # Component unit tests
├── integration/           # Cross-component tests
├── performance/           # Benchmarks and load tests
├── security/             # Security-specific tests
├── simulation/           # Network simulation tests
├── hardware/             # Hardware-in-the-loop tests
├── scenarios/            # Emergency scenario tests
└── fixtures/             # Test data and helpers
```

## 1. Unit Tests

### 1.1 Message Manager Tests

```rust
// tests/unit/message_manager.rs
use disaster_mesh::{Message, MessageManager, MessageContent, UserId};
use tokio_test;

#[tokio::test]
async fn test_message_creation() {
    let manager = MessageManager::new().await.unwrap();
    let sender = UserId::generate();
    let content = MessageContent::Text("Hello World".to_string());
    
    let message = manager.create_message(sender, None, content).await.unwrap();
    
    assert_eq!(message.sender, sender);
    assert!(message.id.is_valid());
    assert!(message.signature.len() > 0);
}

#[tokio::test]
async fn test_message_encryption_decryption() {
    let manager = MessageManager::new().await.unwrap();
    let (sender_key, receiver_key) = generate_test_keypairs();
    
    let original_content = MessageContent::Text("Secret message".to_string());
    let encrypted = manager.encrypt_message(&original_content, &receiver_key.public).await.unwrap();
    let decrypted = manager.decrypt_message(&encrypted, &receiver_key.private).await.unwrap();
    
    assert_eq!(original_content, decrypted);
}

#[tokio::test]
async fn test_message_validation() {
    let manager = MessageManager::new().await.unwrap();
    let invalid_message = create_invalid_message(); // Helper function
    
    let result = manager.validate_message(&invalid_message).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_message_deduplication() {
    let manager = MessageManager::new().await.unwrap();
    let message = create_test_message();
    
    assert!(manager.is_new_message(&message.id).await);
    manager.mark_message_seen(&message.id).await.unwrap();
    assert!(!manager.is_new_message(&message.id).await);
}

#[tokio::test]
async fn test_ttl_expiration() {
    let manager = MessageManager::new().await.unwrap();
    let expired_message = create_expired_message(); // Helper function
    
    let result = manager.validate_message(&expired_message).await;
    assert!(matches!(result, Err(MessageError::Expired)));
}
```

### 1.2 Routing Engine Tests

```rust
// tests/unit/routing_engine.rs
use disaster_mesh::{RoutingEngine, RouteInfo, PeerId, UserId};
use std::time::{SystemTime, Duration};

#[tokio::test]
async fn test_route_discovery() {
    let mut engine = RoutingEngine::new().await;
    let destination = UserId::generate();
    let next_hop = PeerId::generate();
    
    engine.add_route(destination, next_hop, 1, 0.9).await;
    
    let route = engine.find_route(&destination).await.unwrap();
    assert_eq!(route.next_hop, next_hop);
    assert_eq!(route.hop_count, 1);
}

#[tokio::test]
async fn test_route_optimization() {
    let mut engine = RoutingEngine::new().await;
    let destination = UserId::generate();
    let hop1 = PeerId::generate();
    let hop2 = PeerId::generate();
    
    // Add longer route first
    engine.add_route(destination, hop1, 3, 0.7).await;
    // Add shorter, better route
    engine.add_route(destination, hop2, 2, 0.9).await;
    
    let route = engine.find_route(&destination).await.unwrap();
    assert_eq!(route.next_hop, hop2); // Should prefer shorter, better route
}

#[tokio::test]
async fn test_route_expiration() {
    let mut engine = RoutingEngine::new().await;
    let destination = UserId::generate();
    let next_hop = PeerId::generate();
    
    // Add route that expires quickly
    engine.add_route_with_expiry(destination, next_hop, 1, 0.9, Duration::from_millis(100)).await;
    
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    let route = engine.find_route(&destination).await;
    assert!(route.is_none());
}

#[tokio::test]
async fn test_loop_prevention() {
    let mut engine = RoutingEngine::new().await;
    let destination = UserId::generate();
    let our_id = engine.get_node_id();
    
    // Try to add route that would create loop
    let result = engine.add_route_if_valid(destination, our_id, 1, 0.9).await;
    assert!(result.is_err());
}
```

### 1.3 Transport Layer Tests

```rust
// tests/unit/transport.rs
use disaster_mesh::{Transport, MockTransport, TransportEvent};
use mockall::predicate::*;

#[tokio::test]
async fn test_transport_abstraction() {
    let mut mock_transport = MockTransport::new();
    let test_data = b"test message".to_vec();
    let peer_id = PeerId::generate();
    
    mock_transport
        .expect_send()
        .with(eq(peer_id), eq(test_data.clone()))
        .times(1)
        .returning(|_, _| Ok(()));
    
    let result = mock_transport.send(peer_id, test_data).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_transport_broadcast() {
    let mut mock_transport = MockTransport::new();
    let test_data = b"broadcast message".to_vec();
    
    mock_transport
        .expect_broadcast()
        .with(eq(test_data.clone()))
        .times(1)
        .returning(|_| Ok(()));
    
    let result = mock_transport.broadcast(test_data).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_transport_peer_management() {
    let mut mock_transport = MockTransport::new();
    let peer_info = create_test_peer_info();
    
    mock_transport
        .expect_get_peers()
        .returning(move || vec![peer_info.clone()]);
    
    let peers = mock_transport.get_peers();
    assert_eq!(peers.len(), 1);
}
```

### 1.4 Cryptography Tests

```rust
// tests/unit/crypto.rs
use disaster_mesh::crypto::{KeyPair, encrypt_message, decrypt_message, sign_message, verify_signature};

#[test]
fn test_keypair_generation() {
    let keypair1 = KeyPair::generate();
    let keypair2 = KeyPair::generate();
    
    assert_ne!(keypair1.private_key, keypair2.private_key);
    assert_ne!(keypair1.public_key, keypair2.public_key);
}

#[test]
fn test_message_signing_verification() {
    let keypair = KeyPair::generate();
    let message = b"test message";
    
    let signature = sign_message(message, &keypair.private_key).unwrap();
    let is_valid = verify_signature(message, &signature, &keypair.public_key).unwrap();
    
    assert!(is_valid);
}

#[test]
fn test_invalid_signature_rejection() {
    let keypair1 = KeyPair::generate();
    let keypair2 = KeyPair::generate();
    let message = b"test message";
    
    let signature = sign_message(message, &keypair1.private_key).unwrap();
    let is_valid = verify_signature(message, &signature, &keypair2.public_key).unwrap();
    
    assert!(!is_valid);
}

#[test]
fn test_symmetric_encryption() {
    let plaintext = b"secret message";
    let key = generate_symmetric_key();
    
    let ciphertext = encrypt_message(plaintext, &key).unwrap();
    let decrypted = decrypt_message(&ciphertext, &key).unwrap();
    
    assert_eq!(plaintext, &decrypted[..]);
    assert_ne!(plaintext, &ciphertext[..]);
}
```

## 2. Integration Tests

### 2.1 End-to-End Message Flow

```rust
// tests/integration/message_flow.rs
use disaster_mesh::{DisasterMesh, MessageContent};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_local_message_delivery() {
    let mut node1 = DisasterMesh::new_test_node("node1").await.unwrap();
    let mut node2 = DisasterMesh::new_test_node("node2").await.unwrap();
    
    // Connect nodes via mock transport
    connect_test_nodes(&mut node1, &mut node2).await;
    
    let message_content = MessageContent::Text("Hello from node1".to_string());
    node1.send_message(node2.get_user_id(), message_content.clone()).await.unwrap();
    
    // Wait for message delivery
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let received_messages = node2.get_received_messages().await;
    assert_eq!(received_messages.len(), 1);
    assert_eq!(received_messages[0].content, message_content);
}

#[tokio::test]
#[serial]
async fn test_multi_hop_routing() {
    let mut node1 = DisasterMesh::new_test_node("node1").await.unwrap();
    let mut node2 = DisasterMesh::new_test_node("node2").await.unwrap();
    let mut node3 = DisasterMesh::new_test_node("node3").await.unwrap();
    
    // Create chain: node1 <-> node2 <-> node3
    connect_test_nodes(&mut node1, &mut node2).await;
    connect_test_nodes(&mut node2, &mut node3).await;
    
    // Wait for route discovery
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    let message_content = MessageContent::Text("Multi-hop message".to_string());
    node1.send_message(node3.get_user_id(), message_content.clone()).await.unwrap();
    
    // Wait for message delivery
    tokio::time::sleep(Duration::from_millis(1000)).await;
    
    let received_messages = node3.get_received_messages().await;
    assert_eq!(received_messages.len(), 1);
    assert_eq!(received_messages[0].content, message_content);
}

#[tokio::test]
#[serial]
async fn test_store_and_forward() {
    let mut node1 = DisasterMesh::new_test_node("node1").await.unwrap();
    let mut node2 = DisasterMesh::new_test_node("node2").await.unwrap();
    let mut node3 = DisasterMesh::new_test_node("node3").await.unwrap();
    
    // Connect node1 to node2 only
    connect_test_nodes(&mut node1, &mut node2).await;
    
    // Send message to offline node3
    let message_content = MessageContent::Text("Store and forward test".to_string());
    node1.send_message(node3.get_user_id(), message_content.clone()).await.unwrap();
    
    // Message should be stored in node2
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_eq!(node2.get_pending_messages().await.len(), 1);
    
    // Now connect node2 to node3
    connect_test_nodes(&mut node2, &mut node3).await;
    
    // Wait for forwarding
    tokio::time::sleep(Duration::from_millis(1000)).await;
    
    let received_messages = node3.get_received_messages().await;
    assert_eq!(received_messages.len(), 1);
    assert_eq!(received_messages[0].content, message_content);
}
```

### 2.2 Transport Integration Tests

```rust
// tests/integration/transport_integration.rs
use disaster_mesh::{BluetoothTransport, WifiDirectTransport, TransportManager};

#[tokio::test]
#[serial]
async fn test_transport_failover() {
    let mut transport_manager = TransportManager::new();
    
    // Add multiple transports in priority order
    transport_manager.add_transport(Box::new(WifiDirectTransport::new())).await;
    transport_manager.add_transport(Box::new(BluetoothTransport::new())).await;
    
    let test_data = b"failover test".to_vec();
    let peer_id = PeerId::generate();
    
    // Simulate WiFi Direct failure
    transport_manager.simulate_transport_failure("wifi_direct").await;
    
    // Should automatically use Bluetooth
    let result = transport_manager.send(peer_id, test_data).await;
    assert!(result.is_ok());
    
    // Verify Bluetooth was used
    let active_transport = transport_manager.get_active_transport_for_peer(peer_id).await;
    assert_eq!(active_transport.transport_type(), TransportType::Bluetooth);
}

#[tokio::test]
#[serial]
async fn test_transport_auto_reconnection() {
    let mut bluetooth_transport = BluetoothTransport::new();
    bluetooth_transport.start().await.unwrap();
    
    // Simulate connection loss
    bluetooth_transport.simulate_disconnection().await;
    
    // Should automatically attempt reconnection
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    assert!(bluetooth_transport.is_connected().await);
}
```

## 3. Performance Tests

### 3.1 Benchmarks

```rust
// tests/performance/benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use disaster_mesh::{Message, MessageManager, RoutingEngine};

fn bench_message_encryption(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let manager = rt.block_on(MessageManager::new()).unwrap();
    
    let mut group = c.benchmark_group("message_encryption");
    
    for size in [100, 1000, 10000, 100000].iter() {
        let data = vec![0u8; *size];
        let content = MessageContent::File { 
            name: "test.bin".to_string(), 
            data 
        };
        
        group.bench_with_input(BenchmarkId::new("encrypt", size), size, |b, _| {
            b.to_async(&rt).iter(|| async {
                let keypair = KeyPair::generate();
                manager.encrypt_message(black_box(&content), &keypair.public_key).await
            });
        });
    }
    
    group.finish();
}

fn bench_routing_lookup(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut engine = rt.block_on(RoutingEngine::new());
    
    // Populate routing table
    rt.block_on(async {
        for i in 0..1000 {
            let dest = UserId::from_bytes(&i.to_be_bytes());
            let next_hop = PeerId::generate();
            engine.add_route(dest, next_hop, 1, 0.9).await;
        }
    });
    
    c.bench_function("route_lookup", |b| {
        b.to_async(&rt).iter(|| async {
            let dest = UserId::from_bytes(&500u32.to_be_bytes());
            engine.find_route(black_box(&dest)).await
        });
    });
}

fn bench_message_serialization(c: &mut Criterion) {
    let message = create_test_message_large(); // Helper function
    
    c.bench_function("serialize_message", |b| {
        b.iter(|| bincode::serialize(black_box(&message)))
    });
    
    let serialized = bincode::serialize(&message).unwrap();
    c.bench_function("deserialize_message", |b| {
        b.iter(|| bincode::deserialize::<Message>(black_box(&serialized)))
    });
}

criterion_group!(benches, bench_message_encryption, bench_routing_lookup, bench_message_serialization);
criterion_main!(benches);
```

### 3.2 Load Testing

```rust
// tests/performance/load_tests.rs
use tokio::time::{interval, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

#[tokio::test]
#[ignore] // Run with --ignored for load tests
async fn test_message_throughput() {
    let node_count = 10;
    let messages_per_node = 100;
    let mut nodes = Vec::new();
    
    // Create test network
    for i in 0..node_count {
        let node = DisasterMesh::new_test_node(&format!("node{}", i)).await.unwrap();
        nodes.push(node);
    }
    
    // Connect all nodes in mesh topology
    for i in 0..node_count {
        for j in i+1..node_count {
            connect_test_nodes(&mut nodes[i], &mut nodes[j]).await;
        }
    }
    
    let start_time = std::time::Instant::now();
    let sent_count = Arc::new(AtomicU64::new(0));
    let received_count = Arc::new(AtomicU64::new(0));
    
    // Start message sending
    let mut handles = Vec::new();
    for i in 0..node_count {
        let node = nodes[i].clone();
        let sent_counter = sent_count.clone();
        let target_node_id = nodes[(i + 1) % node_count].get_user_id();
        
        let handle = tokio::spawn(async move {
            for j in 0..messages_per_node {
                let content = MessageContent::Text(format!("Message {} from node {}", j, i));
                node.send_message(target_node_id, content).await.unwrap();
                sent_counter.fetch_add(1, Ordering::SeqCst);
                
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        });
        handles.push(handle);
    }
    
    // Wait for all messages to be sent
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Wait for message delivery
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // Count received messages
    let mut total_received = 0;
    for node in &nodes {
        total_received += node.get_received_messages().await.len();
    }
    
    let elapsed = start_time.elapsed();
    let sent_total = sent_count.load(Ordering::SeqCst);
    
    println!("Sent: {}, Received: {}, Time: {:?}", sent_total, total_received, elapsed);
    println!("Throughput: {:.2} msg/sec", total_received as f64 / elapsed.as_secs_f64());
    
    // Assert reasonable delivery rate (>90%)
    assert!(total_received > (sent_total as usize * 9 / 10));
}

#[tokio::test]
#[ignore]
async fn test_memory_usage_under_load() {
    let node = DisasterMesh::new_test_node("memory_test").await.unwrap();
    let initial_memory = get_memory_usage();
    
    // Send large number of messages
    for i in 0..10000 {
        let content = MessageContent::Text(format!("Memory test message {}", i));
        node.queue_message_local(content).await.unwrap();
    }
    
    let peak_memory = get_memory_usage();
    
    // Trigger cleanup
    node.cleanup_old_messages().await.unwrap();
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    let final_memory = get_memory_usage();
    
    println!("Memory usage - Initial: {}MB, Peak: {}MB, Final: {}MB", 
             initial_memory, peak_memory, final_memory);
    
    // Assert memory growth is reasonable
    assert!(peak_memory - initial_memory < 100); // Less than 100MB growth
    assert!(final_memory - initial_memory < 50);  // Cleanup should free most memory
}
```

## 4. Security Tests

### 4.1 Cryptographic Security

```rust
// tests/security/crypto_tests.rs
use disaster_mesh::crypto::*;
use quickcheck_macros::quickcheck;

#[quickcheck]
fn test_signature_never_same_for_different_messages(msg1: Vec<u8>, msg2: Vec<u8>) -> bool {
    if msg1 == msg2 {
        return true; // Skip identical messages
    }
    
    let keypair = KeyPair::generate();
    let sig1 = sign_message(&msg1, &keypair.private_key).unwrap();
    let sig2 = sign_message(&msg2, &keypair.private_key).unwrap();
    
    sig1 != sig2
}

#[quickcheck]
fn test_encryption_different_outputs(msg: Vec<u8>) -> bool {
    if msg.is_empty() {
        return true;
    }
    
    let key = generate_symmetric_key();
    let encrypted1 = encrypt_message(&msg, &key).unwrap();
    let encrypted2 = encrypt_message(&msg, &key).unwrap();
    
    // Due to random nonce, encryptions should be different
    encrypted1 != encrypted2
}

#[test]
fn test_timing_attack_resistance() {
    let keypair = KeyPair::generate();
    let valid_message = b"valid message";
    let invalid_message = b"invalid message";
    
    let valid_signature = sign_message(valid_message, &keypair.private_key).unwrap();
    let invalid_signature = sign_message(invalid_message, &keypair.private_key).unwrap();
    
    // Measure timing for valid and invalid signature verification
    let iterations = 1000;
    
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = verify_signature(valid_message, &valid_signature, &keypair.public_key);
    }
    let valid_time = start.elapsed();
    
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = verify_signature(valid_message, &invalid_signature, &keypair.public_key);
    }
    let invalid_time = start.elapsed();
    
    // Timing should be similar (within 10% to account for noise)
    let ratio = valid_time.as_nanos() as f64 / invalid_time.as_nanos() as f64;
    assert!(ratio > 0.9 && ratio < 1.1, "Potential timing attack vulnerability: ratio = {}", ratio);
}
```

### 4.2 Attack Resistance Tests

```rust
// tests/security/attack_tests.rs
use disaster_mesh::{DisasterMesh, Message, MessageContent};

#[tokio::test]
async fn test_replay_attack_resistance() {
    let mut node1 = DisasterMesh::new_test_node("node1").await.unwrap();
    let mut node2 = DisasterMesh::new_test_node("node2").await.unwrap();
    
    connect_test_nodes(&mut node1, &mut node2).await;
    
    let message_content = MessageContent::Text("Original message".to_string());
    node1.send_message(node2.get_user_id(), message_content).await.unwrap();
    
    // Wait for delivery
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Capture the message
    let transport = node1.get_transport_for_testing();
    let captured_message = transport.get_last_sent_message().await.unwrap();
    
    // Try to replay the message
    let initial_count = node2.get_received_messages().await.len();
    transport.inject_message(captured_message).await;
    
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Should not accept replayed message
    let final_count = node2.get_received_messages().await.len();
    assert_eq!(initial_count, final_count);
}

#[tokio::test]
async fn test_message_flooding_protection() {
    let mut attacker = DisasterMesh::new_test_node("attacker").await.unwrap();
    let mut victim = DisasterMesh::new_test_node("victim").await.unwrap();
    
    connect_test_nodes(&mut attacker, &mut victim).await;
    
    // Flood with messages
    let flood_count = 1000;
    for i in 0..flood_count {
        let content = MessageContent::Text(format!("Flood message {}", i));
        attacker.send_message(victim.get_user_id(), content).await.unwrap();
    }
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    
    // Victim should implement rate limiting
    let received_count = victim.get_received_messages().await.len();
    assert!(received_count < flood_count / 2, "Rate limiting not working: received {}", received_count);
}

#[tokio::test]
async fn test_malformed_message_handling() {
    let mut node = DisasterMesh::new_test_node("test_node").await.unwrap();
    let transport = node.get_transport_for_testing();
    
    // Test various malformed messages
    let malformed_messages = vec![
        vec![0u8; 0],                    // Empty message
        vec![0xDE, 0xAD, 0xBE, 0xEF],   // Invalid serialization
        vec![0u8; 1000000],              // Oversized message
        b"not valid bincode".to_vec(),   // Invalid format
    ];
    
    for malformed in malformed_messages {
        let result = transport.inject_raw_data(malformed).await;
        // Should handle gracefully without crashing
        assert!(result.is_err() || result.is_ok());
    }
    
    // Node should still be functional
    assert!(node.is_healthy().await);
}
```

## 5. Network Simulation Tests

### 5.1 Large Network Simulation

```rust
// tests/simulation/network_simulation.rs
use disaster_mesh::simulation::{NetworkSimulator, SimulationConfig};

#[tokio::test]
#[ignore] // Long-running test
async fn test_large_mesh_network() {
    let config = SimulationConfig {
        node_count: 100,
        connection_probability: 0.1, // Sparse network
        message_rate: 1.0, // 1 message per second per node
        simulation_duration: Duration::from_secs(300), // 5 minutes
        mobility_enabled: false,
    };
    
    let mut simulator = NetworkSimulator::new(config).await;
    let results = simulator.run().await.unwrap();
    
    // Analyze results
    assert!(results.message_delivery_rate > 0.85); // 85% delivery rate
    assert!(results.average_latency < Duration::from_secs(30));
    assert!(results.network_partitions < 5); // Minimal partitioning
    
    println!("Simulation Results:");
    println!("- Delivery Rate: {:.2}%", results.message_delivery_rate * 100.0);
    println!("- Average Latency: {:?}", results.average_latency);
    println!("- Max Hop Count: {}", results.max_hop_count);
}

#[tokio::test]
#[ignore]
async fn test_mobile_network_simulation() {
    let config = SimulationConfig {
        node_count: 50,
        connection_probability: 0.15,
        message_rate: 0.5,
        simulation_duration: Duration::from_secs(600), // 10 minutes
        mobility_enabled: true,
        mobility_speed: 5.0, // 5 m/s average
    };
    
    let mut simulator = NetworkSimulator::new(config).await;
    let results = simulator.run().await.unwrap();
    
    // Mobile networks should still maintain reasonable performance
    assert!(results.message_delivery_rate > 0.75); // 75% delivery rate
    assert!(results.route_discovery_success_rate > 0.80);
}

#[tokio::test]
#[ignore]
async fn test_network_partitioning_recovery() {
    let mut simulator = NetworkSimulator::new_default(20).await;
    
    // Start simulation
    simulator.start().await;
    
    // Run for a while
    tokio::time::sleep(Duration::from_secs(60)).await;
    
    // Create partition
    simulator.partition_network(vec![0, 1, 2, 3, 4], vec![5, 6, 7, 8, 9]).await;
    
    // Run with partition
    tokio::time::sleep(Duration::from_secs(120)).await;
    
    // Heal partition
    simulator.heal_partition().await;
    
    // Run recovery period
    tokio::time::sleep(Duration::from_secs(60)).await;
    
    let results = simulator.stop().await.unwrap();
    
    // Network should recover and deliver stored messages
    assert!(results.partition_recovery_time < Duration::from_secs(30));
    assert!(results.post_recovery_delivery_rate > 0.90);
}
```

## 6. Hardware-in-the-Loop Tests

### 6.1 Bluetooth Hardware Tests

```rust
// tests/hardware/bluetooth_tests.rs
use disaster_mesh::BluetoothTransport;
use serial_test::serial;

#[tokio::test]
#[serial]
#[ignore] // Requires Bluetooth hardware
async fn test_bluetooth_device_discovery() {
    let mut transport = BluetoothTransport::new();
    transport.start().await.unwrap();
    
    // Scan for devices
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    let discovered_peers = transport.get_peers();
    println!("Discovered {} Bluetooth peers", discovered_peers.len());
    
    // Should discover at least some devices in normal environment
    // (This test might fail in isolated environments)
    transport.stop().await.unwrap();
}

#[tokio::test]
#[serial]
#[ignore]
async fn test_bluetooth_connection_establishment() {
    // This test requires two devices running the test
    let mut transport = BluetoothTransport::new();
    transport.start().await.unwrap();
    
    // Try to connect to known test peer
    let test_peer_id = get_test_bluetooth_peer_id(); // Configuration-dependent
    
    if let Some(peer_id) = test_peer_id {
        let result = transport.connect_to_peer(peer_id).await;
        assert!(result.is_ok());
        
        // Test basic communication
        let test_message = b"Hardware test message".to_vec();
        let send_result = transport.send(peer_id, test_message).await;
        assert!(send_result.is_ok());
    }
    
    transport.stop().await.unwrap();
}
```

### 6.2 WiFi Direct Hardware Tests

```rust
// tests/hardware/wifi_direct_tests.rs
use disaster_mesh::WifiDirectTransport;

#[tokio::test]
#[serial]
#[ignore] // Requires WiFi Direct capable hardware
async fn test_wifi_direct_group_formation() {
    let mut transport = WifiDirectTransport::new();
    transport.start().await.unwrap();
    
    // Try to create a group
    let group_result = transport.create_group("DisasterMesh-Test").await;
    
    if group_result.is_ok() {
        println!("WiFi Direct group created successfully");
        
        // Wait for potential connections
        tokio::time::sleep(Duration::from_secs(30)).await;
        
        let connected_peers = transport.get_peers();
        println!("Connected peers: {}", connected_peers.len());
        
        transport.destroy_group().await.unwrap();
    } else {
        println!("WiFi Direct not available or failed to create group");
    }
    
    transport.stop().await.unwrap();
}
```

## 7. Emergency Scenario Tests

### 7.1 Disaster Simulation Tests

```rust
// tests/scenarios/disaster_scenarios.rs
use disaster_mesh::{DisasterMesh, EmergencyLevel, MessageContent};

#[tokio::test]
#[ignore]
async fn test_earthquake_response_scenario() {
    // Simulate post-earthquake communication network
    let survivor_count = 20;
    let responder_count = 5;
    
    let mut survivors = Vec::new();
    let mut responders = Vec::new();
    
    // Create survivor nodes
    for i in 0..survivor_count {
        let node = DisasterMesh::new_with_profile(
            &format!("survivor_{}", i),
            UserProfile::Survivor
        ).await.unwrap();
        survivors.push(node);
    }
    
    // Create responder nodes
    for i in 0..responder_count {
        let node = DisasterMesh::new_with_profile(
            &format!("responder_{}", i),
            UserProfile::EmergencyResponder
        ).await.unwrap();
        responders.push(node);
    }
    
    // Create sparse, damaged network topology
    create_disaster_topology(&mut survivors, &mut responders).await;
    
    // Simulate emergency communications
    let emergency_messages = vec![
        ("Medical emergency at location A", EmergencyLevel::Critical),
        ("Need food and water", EmergencyLevel::High),
        ("Building collapse, people trapped", EmergencyLevel::Critical),
        ("Safe shelter available here", EmergencyLevel::Normal),
    ];
    
    // Send emergency messages
    for (i, (message, level)) in emergency_messages.iter().enumerate() {
        let content = MessageContent::Emergency {
            level: *level,
            details: message.to_string(),
        };
        
        survivors[i % survivor_count]
            .broadcast_emergency_message(content)
            .await
            .unwrap();
    }
    
    // Wait for message propagation
    tokio::time::sleep(Duration::from_secs(60)).await;
    
    // Verify emergency responders received critical messages
    for responder in &responders {
        let emergency_msgs = responder.get_emergency_messages().await;
        let critical_msgs: Vec<_> = emergency_msgs
            .iter()
            .filter(|msg| matches!(msg.get_emergency_level(), EmergencyLevel::Critical))
            .collect();
        
        assert!(critical_msgs.len() > 0, "Responder should receive critical messages");
    }
}

#[tokio::test]
#[ignore]
async fn test_evacuation_coordination_scenario() {
    let mut evacuation_coordinator = DisasterMesh::new_with_profile(
        "coordinator",
        UserProfile::EvacuationCoordinator
    ).await.unwrap();
    
    let evacuee_count = 50;
    let mut evacuees = Vec::new();
    
    for i in 0..evacuee_count {
        let node = DisasterMesh::new_with_profile(
            &format!("evacuee_{}", i),
            UserProfile::Evacuee
        ).await.unwrap();
        evacuees.push(node);
    }
    
    // Create evacuation network
    create_evacuation_topology(&mut evacuation_coordinator, &mut evacuees).await;
    
    // Coordinator sends evacuation instructions
    let evacuation_msg = MessageContent::Emergency {
        level: EmergencyLevel::High,
        details: "Evacuate to designated safe zone Alpha. Follow Route 1.".to_string(),
    };
    
    evacuation_coordinator
        .broadcast_emergency_message(evacuation_msg)
        .await
        .unwrap();
    
    // Wait for message propagation
    tokio::time::sleep(Duration::from_secs(30)).await;
    
    // Verify all evacuees received the message
    let mut received_count = 0;
    for evacuee in &evacuees {
        let messages = evacuee.get_received_messages().await;
        if messages.iter().any(|m| matches!(m.content, MessageContent::Emergency { .. })) {
            received_count += 1;
        }
    }
    
    let delivery_rate = received_count as f64 / evacuee_count as f64;
    assert!(delivery_rate > 0.90, "Evacuation message delivery rate too low: {:.2}%", delivery_rate * 100.0);
}
```

### 7.2 Resource Coordination Tests

```rust
// tests/scenarios/resource_coordination.rs
#[tokio::test]
#[ignore]
async fn test_resource_sharing_scenario() {
    let mut resource_providers = Vec::new();
    let mut resource_seekers = Vec::new();
    
    // Create providers with different resources
    let providers_data = vec![
        ("provider_medical", vec!["first_aid_kit", "antibiotics", "bandages"]),
        ("provider_food", vec!["water", "canned_food", "energy_bars"]),
        ("provider_shelter", vec!["tents", "blankets", "sleeping_bags"]),
    ];
    
    for (name, resources) in providers_data {
        let mut node = DisasterMesh::new_test_node(name).await.unwrap();
        for resource in resources {
            node.register_available_resource(resource, 10).await.unwrap(); // 10 units each
        }
        resource_providers.push(node);
    }
    
    // Create seekers with different needs
    for i in 0..10 {
        let node = DisasterMesh::new_test_node(&format!("seeker_{}", i)).await.unwrap();
        resource_seekers.push(node);
    }
    
    // Connect all nodes
    create_resource_network(&mut resource_providers, &mut resource_seekers).await;
    
    // Seekers request resources
    let resource_requests = vec![
        ("water", 2),
        ("first_aid_kit", 1),
        ("tents", 1),
        ("antibiotics", 1),
    ];
    
    for (i, (resource, quantity)) in resource_requests.iter().enumerate() {
        resource_seekers[i]
            .request_resource(resource, *quantity)
            .await
            .unwrap();
    }
    
    // Wait for resource coordination
    tokio::time::sleep(Duration::from_secs(30)).await;
    
    // Verify resource allocation
    for seeker in &resource_seekers {
        let allocated_resources = seeker.get_allocated_resources().await;
        if !allocated_resources.is_empty() {
            println!("Seeker {} received: {:?}", seeker.get_id(), allocated_resources);
        }
    }
}
```

## 8. Cross-Platform Tests

### 8.1 Platform-Specific Tests

```rust
// tests/platform/cross_platform.rs
#[cfg(target_os = "linux")]
mod linux_tests {
    use disaster_mesh::platform::linux::*;
    
    #[tokio::test]
    async fn test_linux_wifi_direct() {
        let interface = get_wifi_direct_interface().await;
        if let Ok(iface) = interface {
            println!("WiFi Direct interface: {}", iface);
            
            let result = create_p2p_group(&iface, "DisasterMesh-Test").await;
            assert!(result.is_ok() || matches!(result, Err(PlatformError::NotSupported)));
        }
    }
    
    #[tokio::test]
    async fn test_linux_bluetooth_permissions() {
        let has_permissions = check_bluetooth_permissions().await;
        if !has_permissions {
            println!("Warning: Bluetooth permissions not available");
        }
    }
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use disaster_mesh::platform::windows::*;
    
    #[tokio::test]
    async fn test_windows_wifi_direct() {
        let result = initialize_wifi_direct().await;
        match result {
            Ok(_) => println!("WiFi Direct initialized successfully"),
            Err(e) => println!("WiFi Direct initialization failed: {}", e),
        }
    }
}

#[cfg(target_os = "macos")]
mod macos_tests {
    use disaster_mesh::platform::macos::*;
    
    #[tokio::test]
    async fn test_macos_bluetooth() {
        let result = initialize_core_bluetooth().await;
        match result {
            Ok(_) => println!("Core Bluetooth initialized successfully"),
            Err(e) => println!("Core Bluetooth initialization failed: {}", e),
        }
    }
}
```

## 9. Test Data and Helpers

### 9.1 Test Utilities

```rust
// tests/fixtures/test_utils.rs
use disaster_mesh::*;
use tempfile::TempDir;

pub fn create_test_message() -> Message {
    let sender = UserId::generate();
    let content = MessageContent::Text("Test message".to_string());
    
    Message {
        id: MessageId::generate(),
        sender,
        recipient: None,
        content,
        timestamp: SystemTime::now(),
        ttl: Duration::from_secs(3600),
        hop_count: 0,
        signature: vec![0u8; 64], // Mock signature
    }
}

pub fn create_test_keypairs() -> (KeyPair, KeyPair) {
    (KeyPair::generate(), KeyPair::generate())
}

pub async fn connect_test_nodes(node1: &mut DisasterMesh, node2: &mut DisasterMesh) {
    let mock_transport = MockTransport::new();
    node1.add_test_transport(mock_transport.clone()).await;
    node2.add_test_transport(mock_transport).await;
}

pub fn get_memory_usage() -> u64 {
    // Platform-specific memory usage measurement
    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let status = fs::read_to_string("/proc/self/status").unwrap();
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                return parts[1].parse::<u64>().unwrap() / 1024; // Convert to MB
            }
        }
    }
    
    // Fallback for other platforms
    0
}

pub async fn create_disaster_topology(
    survivors: &mut Vec<DisasterMesh>,
    responders: &mut Vec<DisasterMesh>
) {
    // Create sparse, realistic disaster network topology
    let mut rng = rand::thread_rng();
    
    // Connect some survivors to each other (damaged local networks)
    for i in 0..survivors.len() {
        for j in i+1..survivors.len() {
            if rng.gen::<f32>() < 0.2 { // 20% connection probability
                connect_test_nodes(&mut survivors[i], &mut survivors[j]).await;
            }
        }
    }
    
    // Connect responders to some survivors (rescue communication)
    for responder in responders {
        for survivor in survivors.iter_mut() {
            if rng.gen::<f32>() < 0.3 { // 30% connection probability
                connect_test_nodes(responder, survivor).await;
            }
        }
    }
}
```

## 10. Continuous Integration

### 10.1 CI Configuration

```yaml
# .github/workflows/test.yml
name: Test Suite

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  unit-tests:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
        rust: [stable, beta]
    
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
        components: rustfmt, clippy
    
    - name: Cache cargo dependencies
      uses: actions/cache@v3
      with:
        path: ~/.cargo
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Run unit tests
      run: cargo test --lib
    
    - name: Run integration tests
      run: cargo test --test integration
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Check formatting
      run: cargo fmt -- --check

  performance-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Run benchmarks
      run: cargo bench --no-run
    
    - name: Run load tests
      run: cargo test --test performance --ignored

  security-tests:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Install cargo-audit
      run: cargo install cargo-audit
    
    - name: Security audit
      run: cargo audit
    
    - name: Run security tests
      run: cargo test --test security
```

## 11. Test Execution Guidelines

### Running Tests

```bash
# Run all unit tests
cargo test --lib

# Run integration tests
cargo test --test integration

# Run performance tests (long-running)
cargo test --test performance --ignored

# Run security tests
cargo test --test security

# Run hardware tests (requires hardware)
cargo test --test hardware --ignored

# Run scenario tests (very long-running)
cargo test --test scenarios --ignored

# Run benchmarks
cargo bench

# Run with coverage
cargo tarpaulin --out Html
```

### Test Categories

- **Fast Tests** (~1 minute): Unit tests, basic integration tests
- **Medium Tests** (~10 minutes): Performance tests, simulation tests
- **Slow Tests** (~1 hour): Hardware tests, large scenario tests
- **Hardware Tests**: Require specific hardware (Bluetooth, WiFi)
- **Manual Tests**: Require human intervention or special setup

### Test Environment Setup

```bash
# Install test dependencies
sudo apt-get install bluetooth bluez-tools

# Set up test configuration
cp config/test.toml.example config/test.toml

# Enable hardware testing (requires permissions)
sudo usermod -a -G bluetooth $USER

# Set up test network interfaces
sudo modprobe dummy
sudo ip link add dummy0 type dummy
```

## 12. Fuzzing & Mutational Testing

| Target | Tool | Harness Path | Notes |
|--------|------|--------------|-------|
| Envelope deserialization | `cargo-fuzz` (libFuzzer) | `fuzz_targets/deserialize_envelope.rs` | Random byte streams -> `Envelope::try_from()` |
| Noise handshake parser | `cargo-fuzz` | `fuzz_targets/noise_handshake.rs` | Ensures no panics / UB on malformed inputs |
| Routing table operations | `honggfuzz` | `fuzz_targets/routing_table.rs` | Focus on memory leaks & OOB writes |

Fuzzing jobs run **nightly** in CI; crashes create GitHub issues with reproducer inputs stored under `artifacts/fuzz-corpus`.

### Running Fuzz Locally
```bash
cargo install cargo-fuzz
cargo fuzz run deserialize_envelope
```

## 13. Simulation (SimNet) CI Job

A docker-compose environment spins up 10 `disastermesh-simnode` containers linked via Linux TC to emulate 100-500 ms latency and 1 % packet loss.  The network runs for 120 seconds and must meet:

- Delivery rate ≥ 90 % of generated messages
- Average latency ≤ 7 s

Failure aborts the pipeline.

## 14. Test KPIs / Success Criteria

| Category | KPI | Threshold |
|----------|-----|-----------|
| Unit | Code coverage (lines) | ≥ 85 % |
| Integration | End-to-end delivery success | ≥ 95 % |
| Performance | Throughput (10 nodes mesh) | ≥ 200 msg/min total |
| Security | Replay-attack acceptance | 0 per 1 000 attempts |
| Fuzzing | Crash-free execs | ≥ 1 billion cycles |
| Simulation | Delivery rate (10-node, 5-hop) | ≥ 90 % |

## 15. Coverage & Static Analysis

- **Coverage**: `cargo tarpaulin --out Xml --fail-under 85` enforced in CI.
- **Sanitizers**: nightly Address & Thread Sanitizer builds (`RUSTFLAGS="-Zsanitizer=address"`).
- **Clippy pedantic**: `cargo clippy --all-targets --all-features -D warnings -W clippy::pedantic`.
- **Dead code**: `cargo machete` pass to prune unused functions.

## 16. Future Hardware Test Expansion

- **LoRa**: Add SX127x evaluation boards; basic join & data-rate tests.
- **HAM Radio (FSK/AX.25)**: SDR-based loopback harness with virtual audio cables.
- **Battery Drain**: Android emulator + Battery Historian scripts to measure energy use during 1h chat session.

This comprehensive testing plan ensures all aspects of the DisasterMesh application are thoroughly validated, from basic functionality to complex emergency scenarios.