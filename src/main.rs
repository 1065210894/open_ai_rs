use async_openai::{Client, types::{CreateCompletionRequestArgs}};

#[tokio::main]
async fn main() {
    let api_key = "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"; // This secret could be from a file, or environment variable.
    let client = Client::new().with_api_key(api_key);

    // Create request using builder pattern
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt("我插你苦胆这句台词是葛优哪部电影的")
        .max_tokens(2048_u16)
        .build()
        .unwrap();

    // Call API
    let response = client
        .completions()      // Get the API "group" (completions, images, etc.) from the client
        .create(request)    // Make the API call in that "group"
        .await
        .unwrap();

    println!("{}", response.choices.first().unwrap().text);
}


