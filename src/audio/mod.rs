use std::sync::Arc;

use cpal::{traits::{DeviceTrait, HostTrait}, Device, Host, StreamConfig, SupportedStreamConfig};
use graph::AudioGraph;

pub mod error;
pub mod graph;

pub struct AudioThread {
    shared_graph: Arc<Option<AudioGraph>>,
    host: Host,
    device: Device,
    config: SupportedStreamConfig,
}

impl AudioThread {
    pub fn new(shared_graph: Arc<Option<AudioGraph>>) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device detected");
        let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
        let config = supported_configs_range.next()
            .expect("no supported config")
            .with_max_sample_rate();
        Self {
            shared_graph,
            host,
            device,
            config,
        }
    }

    pub fn run(&self) {
        let stream = self.device.build_output_stream(
            &self.config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // react to stream events and read or write stream data here.
            },
            move |err| {
                // react to errors here.
            },
            None // None=blocking, Some(Duration)=timeout
        ).expect("failed to build output stream");
    }
}