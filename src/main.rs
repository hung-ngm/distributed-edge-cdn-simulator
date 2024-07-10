use std::sync::Arc;
use tokio::sync::Mutex;
use distributed_edge_cdn_simulator::{
    NetworkSimulator,
    CDN,
    EdgeNode,
    Content,
    Request   
};

#[tokio::main]
async fn main() {
    let cdn = Arc::new(Mutex::new(CDN::new()));
    let network = NetworkSimulator::new(50, 20);

    // Initialize CDN with some edge nodes
    {
        let mut cdn = cdn.lock().await;
        cdn.add_edge_node(EdgeNode {
            id: "node1".to_string(),
            capacity: 1000,
            stored_content: vec![]
        });
        cdn.add_edge_node(EdgeNode {
            id: "node2".to_string(),
            capacity: 1000,
            stored_content: vec![]
        });
    }

    // Distribute some content
    {
        let mut cdn = cdn.lock().await;
        for i in 0..10 {
            let content = Content {
                id: format!("video{}", i),
                size: 100,
            };
            cdn.optimized_distribute_content(content, &network).await;
        }
    }

    // Handles requests concurrently
    let mut handles = vec![];
    for i in 0..100 {
        let cdn_clone = cdn.clone();
        let network_clone = network.clone();
        let handle = tokio::spawn(async move {
            let request = Request {
                content_id: format!{"video{}", i % 10},
                origin: format!("user{}", i),
            };
            let cdn = cdn_clone.lock().await;
            cdn.handle_request(request, &network_clone).await
        });
        handles.push(handle);
    }

    for handle in handles {
        if let Ok(Some(node)) = handle.await {
            println!("Content served by node: {}", node);
        } else {
            println!("Content not found");
        }
    }
}