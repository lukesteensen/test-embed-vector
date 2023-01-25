use vector::{
    config::{DataType, GenerateConfig, Input, Output, TransformConfig, TransformContext},
    event::Event,
    schema,
    transforms::{FunctionTransform, OutputBuffer, Transform},
};
use vector_config::configurable_component;
use vector_core::config::LogNamespace;

/// Configuration for the `special_sample` transform.
#[configurable_component(transform("special_sample"))]
#[derive(Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SampleConfig {}

impl GenerateConfig for SampleConfig {
    fn generate_config() -> toml::Value {
        toml::Value::try_from(Self {}).unwrap()
    }
}

#[async_trait::async_trait]
impl TransformConfig for SampleConfig {
    async fn build(&self, _context: &TransformContext) -> vector::Result<Transform> {
        Ok(Transform::function(Sample))
    }

    fn input(&self) -> Input {
        Input::new(DataType::Log)
    }

    fn outputs(&self, merged_definition: &schema::Definition, _: LogNamespace) -> Vec<Output> {
        vec![Output::default(DataType::Log).with_schema_definition(merged_definition.clone())]
    }
}

#[derive(Clone)]
pub struct Sample;

impl FunctionTransform for Sample {
    fn transform(&mut self, output: &mut OutputBuffer, mut event: Event) {
        let log = event.as_mut_log();
        log.insert("enterprise_only", "special_sauce");
        output.push(event);
    }
}
