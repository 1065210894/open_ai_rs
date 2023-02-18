use async_openai::{types::CreateCompletionRequestArgs, Client};

pub async fn send_open_ai_request(content: &str) {
    let api_key = "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"; // This secret could be from a file, or environment variable.
    let client = Client::new().with_api_key(api_key);

    // Create request using builder pattern
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(content)
        .max_tokens(2048_u16)
        .temperature(0.9)
        .build()
        .unwrap();

    // Call API
    let response = client
        .completions() // Get the API "group" (completions, images, etc.) from the client
        .create(request) // Make the API call in that "group"
        .await
        .unwrap();

    println!("{}", response.choices.first().unwrap().text);
}
