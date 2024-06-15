use std::cmp::max;

use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
};

use crate::utils::preprocess::{chunk_on_delimiter, get_chat_completion, tokenize};

pub async fn summarize(
    input_str: &String,
    detail: Option<f64>,
    model: Option<&str>,
    additional_instruction: Option<&str>,
    minimal_chunk_size: Option<usize>,
    chunk_delimiter: Option<&str>,
    summarize_recursively: Option<bool>,
    verbose: Option<bool>,
) -> String {
    //! Summarizes a given text by splitting it into chunks, each of which is summarized individually.
    //! The level of detail in the summary can be adjusted, and the process can optionally be made recursive.
    //!
    //! The function first determines the number of chunks by interpolating between a minimum and a maximum chunk count based on the `detail` parameter.
    //! It then splits the text into chunks and summarizes each chunk. If `summarize_recursively` is True, each summary is based on the previous summaries,
    //! adding more context to the summarization process. The function returns a compiled summary of all chunks.
    //!

    // Assign default value first
    let detail = detail.unwrap_or(0.0);
    let model = model.unwrap_or("gpt-4-turbo");
    let minimal_chunk_size = minimal_chunk_size.unwrap_or(500);
    let chunk_delimiter = chunk_delimiter.unwrap_or(".");
    let summarize_recursively = summarize_recursively.unwrap_or(false);
    let verbose = verbose.unwrap_or(false);

    if detail < 0.0 || detail > 1.0 {
        panic!("invalid detail");
    }

    let max_chunks = chunk_on_delimiter(input_str, minimal_chunk_size, chunk_delimiter).len();
    let min_chunks: usize = 1;
    let num_chunks =
        (min_chunks + (((max_chunks - min_chunks) as f64) * detail).ceil() as usize) as usize;

    // Adjust chunk_size based on interpolated number of chunks
    let document_length = tokenize(input_str).len();
    let chunk_size = max(minimal_chunk_size, document_length / num_chunks);
    let text_chunks = chunk_on_delimiter(input_str, chunk_size, chunk_delimiter);
    if verbose {
        println!(
            "Splitting the text into {} chunks to be summarized.",
            text_chunks.len()
        );
    }
    let mut system_message_content = String::from("Rewrite this text in summarized form.");
    if additional_instruction.is_some() {
        system_message_content.push_str("\n\n");
        system_message_content.push_str(additional_instruction.unwrap());
    }
    let mut accumulated_summaries: Vec<String> = vec![];

    for chunk in text_chunks {
        let mut user_message = String::from("");
        if summarize_recursively {
            let accumulated_summaries_string = accumulated_summaries.join("\n\n");
            user_message.push_str("Previous summaries:\n\n");
            user_message.push_str(accumulated_summaries_string.as_str());
            user_message.push_str("\n\nText to summarize next:\n\n");
        }
        user_message.push_str(chunk.clone().as_str());

        // build the body of request
        let system_message: ChatCompletionRequestMessage = ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(&system_message_content)
                .build()
                .unwrap(),
        );

        let user_message: ChatCompletionRequestMessage = ChatCompletionRequestMessage::User(
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_message)
                .build()
                .unwrap(),
        );
        let messages = vec![system_message, user_message];

        let response = get_chat_completion(messages, model).await;
        accumulated_summaries.push(response);
    }
    // Compile final summary from partial summaries
    let final_summary = accumulated_summaries.join("\n\n");

    return final_summary;
}
