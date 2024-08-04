variable "project_id" {
  type = string
}

variable "postgres_username" {
  type = string
}

variable "postgres_password" {
  type = string
}
variable "postgres_host" {
  type = string
}

variable "postgres_db" {
  type = string
}


resource "google_secret_manager_secret" "news_summarizer_db_credentials" {
  project   = var.project_id
  secret_id = "news-summarizer-db-credentials"
  replication {
    auto {
    }
  }
}

resource "google_secret_manager_secret_version" "postgres_user" {
  secret      = google_secret_manager_secret.news_summarizer_db_credentials.id
  secret_data = var.postgres_username
}

resource "google_secret_manager_secret_version" "postgres_password" {
  secret      = google_secret_manager_secret.news_summarizer_db_credentials.id
  secret_data = var.postgres_password
}
resource "google_secret_manager_secret_version" "postgres_db" {
  secret      = google_secret_manager_secret.news_summarizer_db_credentials.id
  secret_data = var.postgres_db
}
resource "google_secret_manager_secret_version" "postgres_host" {
  secret      = google_secret_manager_secret.news_summarizer_db_credentials.id
  secret_data = var.postgres_host
}

# Output the secret name and project ID
output "secret_name" {
  value = google_secret_manager_secret.news_summarizer_db_credentials.secret_id
}

output "project_id" {
  value = google_secret_manager_secret.news_summarizer_db_credentials.project
}
