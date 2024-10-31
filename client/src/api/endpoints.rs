use crate::api::client::ApiService;
use std::error::Error;

use super::schemas::{AllowUserInput, AuthResponse, GetKeysInput, ProjectInput};

pub async fn check_health(api_service: &ApiService) -> Result<(), Box<dyn Error>> {
    let health_status = api_service.get_health().await?;
    println!("Health Status: {}", health_status);
    Ok(())
}

pub async fn create_account(
    api_service: &ApiService,
    name: &str,
    email: &str,
    password: &str,
) -> Result<(), Box<dyn Error>> {
    match api_service.create_account(name, email, password).await {
        Ok(created_account) => {
            println!("Account created successfully!");
            println!("{:?}", created_account);
            Ok(())
        }
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during account creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn authenticate(
    api_service: &ApiService,
    email: &str,
    password: &str,
) -> Result<AuthResponse, Box<dyn Error>> {
    match api_service.get_authentication_token(email, password).await {
        Ok(token) => Ok(token),
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during account creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn create_project(
    api_service: &ApiService,
    input: ProjectInput,
) -> Result<serde_json::Value, Box<dyn Error>> {
    match api_service.create_project(input).await {
        Ok(project) => {
            println!("Project created successfully!");
            println!("{:?}", project);
            Ok(project)
        }
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during project creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn add_allowed_user(
    api_service: &ApiService,
    input: AllowUserInput,
) -> Result<serde_json::Value, Box<dyn Error>> {
    let user_email = input.user_email.clone();
    let project_name = input.project_name.clone();

    match api_service.add_allowed_user(input).await {
        Ok(response) => {
            println!("Added {:?} to {:?} project", user_email, project_name);
            Ok(response)
        }
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during project creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn generate_qrcode_file(
    api_service: &ApiService,
    project_name: String,
) -> Result<(), Box<dyn Error>> {
    match api_service.generate_qrcode_file(project_name).await {
        Ok(response) => {
            println!("Created qrcode file");
            Ok(response)
        }
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during project creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn build_project(
    api_service: &ApiService,
    input: GetKeysInput,
) -> Result<serde_json::Value, Box<dyn Error>> {
    match api_service.build_project(input).await {
        Ok(response) => Ok(response),
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during project creation: {}", e);
            }
            Err(e)
        }
    }
}

pub async fn get_all_projects(
    api_service: &ApiService,
) -> Result<serde_json::Value, Box<dyn Error>> {
    match api_service.get_all_projects().await {
        Ok(response) => Ok(response),
        Err(e) => {
            if e.is::<reqwest::Error>() {
                eprintln!("Failed to communicate with the server: {}", e);
            } else if e.is::<serde_json::Error>() {
                eprintln!("Error parsing the server's response: {}", e);
            } else {
                eprintln!("An error occurred during project creation: {}", e);
            }
            Err(e)
        }
    }
}
