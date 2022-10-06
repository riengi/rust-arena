// Text generation model using rust_bert and GPT_NEO AI model
// Inspired by https://www.youtube.com/watch?v=StMP7g-0wK4
// Warning: about 10Gig of GPT model data will be downloaded!!!

use rust_bert::{
    gpt_neo::{
        GptNeoConfigResources, GptNeoMergesResources, GptNeoModelResources, GptNeoVocabResources,
    },
    pipelines::{
        common::ModelType,
        text_generation::{TextGenerationConfig, TextGenerationModel},
    },
    resources::RemoteResource,
};

fn main() {
    println!("Rust bert AI text generation, input text / hidden context:");
    let model_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B,
    ));

    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));
    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));
    let merges_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoMergesResources::GPT_NEO_2_7B,
    ));

    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: 128,
        ..Default::default()
    };

    let model = TextGenerationModel::new(generate_config).unwrap();

    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let split = line.split('/').collect::<Vec<&str>>();
        let slc = split.as_slice();
        let output = model.generate(&slc[1..], Some(slc[0]));
        for sentence in output {
            println!("{}", sentence);
        }
    }
}
