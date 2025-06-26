use disaster_mesh::{routing_control::RoutingControl, UserId};

#[test]
fn test_routing_control_roundtrip() {
    let origin = UserId::random();
    let dest = UserId::random();
    let packet = RoutingControl::Rreq {
        origin,
        destination: dest,
        request_id: 42,
        hop_count: 0,
    };

    // Serialize with bincode then deserialize
    let encoded = bincode::serialize(&packet).expect("serialize");
    let decoded: RoutingControl = bincode::deserialize(&encoded).expect("deserialize");

    assert_eq!(packet, decoded);
} 