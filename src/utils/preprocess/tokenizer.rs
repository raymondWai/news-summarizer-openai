use async_openai::{
    types::{ChatCompletionRequestMessage, CreateChatCompletionRequestArgs},
    Client,
};
use tiktoken_rs::p50k_base;

pub fn tokenize(prompt: &String) -> Vec<usize> {
    let bpe = p50k_base().unwrap();
    let tokens = bpe.encode_with_special_tokens(&prompt);
    tokens
}

fn combine_chunks_with_no_minimun<'a>(
    chunks: &Vec<&'a str>,
    max_token: usize,
    chunk_delimer: &str,
    header: Option<&str>,
    add_ellipsis_for_overflow: bool,
) -> (Vec<String>, Vec<Vec<usize>>, usize) {
    // This function combines text chunks into larger blocks without exceeding a specified token count. It returns the combined text blocks, their original indices, and the count of chunks dropped due to overflow.
    let dropped_chunk_count = 0;
    let mut output: Vec<String> = vec![]; // list to hold combined chunks
    let mut output_indices: Vec<Vec<usize>> = vec![]; //lst top hold theindics of final combined chunks

    let mut candidate: Vec<&str> = match header {
        Some(c) => vec![c],
        None => vec![],
    };
    let mut candidate_indices: Vec<usize> = vec![];
    for chunk_index in 0..chunks.len() {
        let chunk_with_header = match header {
            Some(h) => vec![h, chunks[chunk_index]],
            None => vec![chunks[chunk_index]],
        };
        let joined_chunks = chunk_with_header.join(chunk_delimer);
        let tokenized_joined_chunks = tokenize(&joined_chunks);
        if tokenized_joined_chunks.len() > max_token {
            println!("warning: chunk overflow");
            // let joined_chunks = [chunk_with_header, vec!["..."]].concat();
            let joined_chunks = chunk_with_header.join(chunk_delimer);
            let tokenized_joined_chunks = tokenize(&joined_chunks);
            if add_ellipsis_for_overflow && tokenized_joined_chunks.len() <= max_token {
                candidate.push("...");
                continue;
            }
        }
        let joined_chunks = [candidate.clone(), vec![chunks[chunk_index]]]
            .concat()
            .join(chunk_delimer);
        let tokenized_joined_chunks = tokenize(&joined_chunks);
        let extended_candidate_token_count = tokenized_joined_chunks.len();
        if extended_candidate_token_count > max_token {
            let joined_candidate = candidate.join(chunk_delimer);
            output.push(joined_candidate.clone());
            output_indices.push(candidate_indices);
            candidate = chunk_with_header;
            candidate_indices = vec![chunk_index];
        } else {
            candidate.push(chunks[chunk_index]);
            candidate_indices.push(chunk_index);
        }
    }
    if (header.is_some() && candidate.len() > 1) || (header.is_none() && candidate.len() > 0) {
        output.push(candidate.join(chunk_delimer));
        output_indices.push(candidate_indices);
    }

    (output, output_indices, dropped_chunk_count)
}

pub fn chunk_on_delimiter(input_str: &String, max_token: usize, delimiter: &str) -> Vec<String> {
    // This function chunks a text into smaller pieces based on a maximum token count and a delimiter.

    let chunks = input_str.as_str().split(delimiter).collect::<Vec<&str>>();
    let (init_combined_chunks, _, dropped_chunk_count) =
        combine_chunks_with_no_minimun(&chunks, max_token, delimiter, None, true);
    if dropped_chunk_count > 0 {
        println!(
            "warning: {} chunks were dropped due to overflow",
            dropped_chunk_count
        );
    }
    let mut combined_chunks = vec![];
    for chunk in init_combined_chunks {
        let mut combined_chunk = String::from(chunk);
        combined_chunk.push_str(delimiter);
        combined_chunks.push(combined_chunk);
    }
    combined_chunks
}

pub async fn get_chat_completion(
    messages: Vec<ChatCompletionRequestMessage>,
    model: &str,
) -> String {
    // Create client
    let client = Client::new();

    // Create request using builder pattern
    // Every request struct has companion builder struct with same name + Args suffix
    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .max_tokens(4000_u16)
        .temperature(0.0)
        .build()
        .unwrap();

    // Call API
    let response = client
        .chat()
        .create(request) // Make the API call in that "group"
        .await
        .unwrap();

    return response
        .choices
        .first()
        .unwrap()
        .message
        .content
        .clone()
        .unwrap()
        .clone();
}
