// ai/model.rs

use rust_bert::pipelines::text_generation::{TextGenerationModel, TextGenerationConfig};

pub struct DungeonMaster {
    model: TextGenerationModel,
}

impl DungeonMaster {
    pub fn new(model_path: &str) -> Self {
        let config = TextGenerationConfig {
            max_length: 100,
            do_sample: true,
            num_beams: 1,
            temperature: 1.0,
            ..Default::default()
        };
        let model = TextGenerationModel::new(config).unwrap();
        // let model = TextGenerationModel::new(model_path, config).unwrap();
        Self { model }
    }

    pub fn generate_response(&self, prompt: &str) -> String {
        // let config = TextGenerationConfig {
        //     max_length: 100,
        //     ..Default::default()
        // };

        // let output = self.model.generate(&[prompt], Some(config)).unwrap();
        // output[0].text.clone()

        let output = self.model.generate(&[prompt], None);
        output[0].to_string()
    }
}