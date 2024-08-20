use std::sync::Arc;

pub struct AudioGraph {
    master_node: AudioNode
}

pub struct AudioNode {
    node_type: NodeType,
    children: Vec<Arc<AudioNode>>
}

pub enum NodeType {
    Output,
    Oscillator,
    Amplifier,
}