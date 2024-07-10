use rand::Rng;

#[derive(Clone, Debug)]
pub struct Content {
    pub id: String,
    pub size: usize,
}

#[derive(Debug)]
pub struct EdgeNode {
    pub id: String,
    pub capacity: usize,
    pub stored_content: Vec<Content>,
}

#[derive(Debug)]
pub struct Request {
    pub content_id: String,
    pub origin: String,
}

pub struct CDN {
    edge_nodes: Vec<EdgeNode>,
}

#[derive(Clone)]
pub struct NetworkSimulator {
    base_latency: u64,
    jitter: u64,
}

impl NetworkSimulator {
    pub fn new(base_latency: u64, jitter: u64) -> Self {
        NetworkSimulator { base_latency, jitter }
    }

    pub async fn simulate_delay(&self) {
        let jitter = rand::thread_rng().gen_range(0..=self.jitter);
        let delay = self.base_latency + jitter;
        tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
    }
}

impl CDN {
    pub fn new() -> Self {
        CDN { edge_nodes: Vec::new() }
    }

    pub fn add_edge_node(&mut self, node: EdgeNode) {
        self.edge_nodes.push(node);
    }

    pub async fn handle_request(&self, request: Request, network: &NetworkSimulator) -> Option<String> {
        network.simulate_delay().await;
        for node in &self.edge_nodes {
            if node.stored_content.iter().any(|c| c.id == request.content_id) {
                return Some(node.id.clone());
            }
        }
        None
    }

    pub fn choose_best_node(&self, content: &Content) -> Option<&EdgeNode> {
        self.edge_nodes
            .iter()
            .filter(|node| node.capacity >= content.size)
            .min_by_key(|node| node.stored_content.len())
    }

    pub async fn optimized_distribute_content(&mut self, content: Content, network: &NetworkSimulator) {
        if let Some(best_node_id) = self.choose_best_node(&content).map(|node| node.id.clone()) {
            if let Some(node) = self.edge_nodes.iter_mut().find(|n| n.id == best_node_id) {
                node.stored_content.push(content.clone());
                node.capacity -= content.size;
                network.simulate_delay().await;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_content_distribution() {
        let mut cdn = CDN::new();
        let network = NetworkSimulator::new(50, 20);

        cdn.add_edge_node(EdgeNode {
            id: "node1".to_string(),
            capacity: 1000,
            stored_content: vec![],
        });

        let content = Content {
            id: "video1".to_string(),
            size: 100,
        };

        cdn.optimized_distribute_content(content.clone(), &network).await;

        assert_eq!(cdn.edge_nodes[0].stored_content.len(), 1);
        assert_eq!(cdn.edge_nodes[0].stored_content[0].id, "video1");
    }

    #[tokio::test]
    async fn test_request_handling() {
        let mut cdn = CDN::new();
        let network = NetworkSimulator::new(50, 20);

        cdn.add_edge_node(EdgeNode {
            id: "node1".to_string(),
            capacity: 1000,
            stored_content: vec![Content {
                id: "video1".to_string(),
                size: 100,
            }],
        });

        let request = Request {
            content_id: "video1".to_string(),
            origin: "user1".to_string(),
        };

        let result = cdn.handle_request(request, &network).await;
        assert_eq!(result, Some("node1".to_string()));
    }
}