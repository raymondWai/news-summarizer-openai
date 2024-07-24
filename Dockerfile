# Use the official Rust image as the base
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release --name

# Expose port 80 for the server
EXPOSE 80

# Run the server application
CMD ["target/release/news-summarizer-openai"]