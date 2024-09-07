use std::env;

fn get_api_url() -> String {
    let api_url = env::var("API_URL").expect("API_URL must be set");

    if !api_url.starts_with("http://") && !api_url.starts_with("https://") {
        panic!("API_URL must start with 'http://' or 'https://'");
    }

    api_url
}

fn get_api_port() -> u16 {
    let port_str = env::var("API_PORT").expect("API_PORT must be set");

    match port_str.parse::<u16>() {
        Ok(port) => port,
        Err(_) => panic!("API_PORT must be a valid u16"),
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use reqwest::StatusCode;
    use serde_json::json;
    use tokio;

    use crate::{get_api_port, get_api_url};

    fn api_url(endpoint: &str) -> String {
        format!(
            "{}:{}/api/v1/{}",
            "http://localhost",
            get_api_port(),
            endpoint
        )
    }

    #[tokio::test]
    async fn test_create_tag_success() {
        dotenv().ok();

        // Corps de la requête JSON pour créer un tag valide
        let body = json!({
            "name": "Technology",
            "slug": "technology",
            "description": "Articles about technology"
        });

        // Envoyer une requête POST à l'API
        let client = reqwest::Client::new();
        let response = client
            .post(&api_url("tags"))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request");

        // Vérifier que la réponse a un statut 201 Created
        assert_eq!(response.status(), StatusCode::CREATED);

        // Vérifier que le corps de la réponse contient les données correctes
        let response_json: serde_json::Value =
            response.json().await.expect("Invalid JSON");
        assert_eq!(response_json["name"], "Technology");
        assert_eq!(response_json["slug"], "technology");
    }

    #[tokio::test]
    async fn test_create_tag_missing_name() {
        dotenv().ok();

        let body = json!({
            "slug": "tech",
            "description": "Articles about technology"
        });

        let client = reqwest::Client::new();
        let response = client
            .post(&api_url("tags"))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let response_json: serde_json::Value =
            response.json().await.expect("Invalid JSON");

        // Comparer le message d'erreur retourné par l'API
        let error_message = response_json["error"].as_str().unwrap();
        assert!(error_message.contains("missing field"));
    }

    #[tokio::test]
    async fn test_create_tag_invalid_type() {
        dotenv().ok();

        // Corps de la requête JSON avec un type incorrect pour le champ "name"
        let body = json!({
            "name": 123, // Devrait être une chaîne de caractères
            "slug": "tech",
            "description": "Articles about technology"
        });

        // Envoyer une requête POST à l'API
        let client = reqwest::Client::new();
        let response = client
            .post(&api_url("tags"))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request");

        // Vérifier que la réponse a un statut 400 Bad Request
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // Vérifier le message d'erreur
        let response_json: serde_json::Value =
            response.json().await.expect("Invalid JSON");
        let error_message = response_json["error"].as_str().unwrap();
        assert!(error_message.contains("invalid type"));
    }
}
