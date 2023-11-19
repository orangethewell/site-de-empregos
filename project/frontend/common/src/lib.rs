use serde::{Deserialize, Serialize};
use reqwasm::http::{Request, RequestCredentials, Method};
use serde_wasm_bindgen::to_value;
use serde_json::to_string as jsonify;
use wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Job {
    pub activities: Vec<String>,
    pub branch: String,
    pub company: String,
    pub id: i32,
    pub opportunities: i32,
    pub requirements: Vec<String>,
    pub title: String
}

pub fn test_encoding<T: Serialize>(object: &T) -> JsValue {
    to_value(&object).unwrap()
}

pub async fn add_job(new_job: &Job) -> Result<(), String> {
    let url = "http://www.vagasemaraxa.com:5000/api/jobs/";
    let request = Request::post(&url)
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .body(to_value(&jsonify(new_job).unwrap()).unwrap());

    let response = request.send().await.map_err(|e| e.to_string())?;

    if response.status() == 200 {
        Ok(())
    } else {
        Err(format!("Failed to add job: {:?}", response))
    }
}

// Função para recuperar todos os trabalhos
pub async fn get_jobs() -> Result<Vec<Job>, String> {
    let url = "http://www.vagasemaraxa.com:5000/api/jobs/";
    let request = Request::get(&url);

    let response = request.send().await.map_err(|e| e.to_string())?;

    if response.status() == 200 {
        let jobs: Vec<Job> = response.json().await.map_err(|e| e.to_string())?;
        Ok(jobs)
    } else {
        Err(format!("Failed to get jobs: {:?}", response))
    }
}

pub async fn get_jobs_count() -> i32 {
    let job_payload = Request::get("http://www.vagasemaraxa.com:5000/api/jobs/count")
        .send().await.unwrap();

    let data: i32 = job_payload.json().await.unwrap();
    data
}

// Função para atualizar um trabalho pelo ID
pub async fn update_job(id: i32, job_details: &Job) -> Result<(), String> {
    let url = format!("http://www.vagasemaraxa.com:5000/api/jobs/{}", id);
    let request = Request::patch(&url)
        .header("Content-Type", "application/json")
        .credentials(RequestCredentials::Include)
        .body(to_value(&jsonify(job_details).unwrap()).unwrap());

    let response = request.send().await.map_err(|e| e.to_string())?;

    if response.status() == 200 {
        Ok(())
    } else {
        Err(format!("Failed to update job: {:?}", response))
    }
}

// Função para excluir um trabalho pelo ID
pub async fn delete_job(id: i32) -> Result<(), String> {
    let url = format!("http://www.vagasemaraxa.com:5000/api/jobs/{}", id);
    let request = Request::delete(&url).credentials(RequestCredentials::Include);

    let response = request.send().await.map_err(|e| e.to_string())?;

    if response.status() == 200 {
        Ok(())
    } else {
        Err(format!("Failed to delete job: {:?}", response))
    }
}


#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    message: String
}

pub async fn login_user(email: String, password: String) -> Result<User, String> {
    #[derive(Serialize, Deserialize)]
    struct LoginForm {
        email: String,
        password: String
    }
    
    let login_payload = Request::post("http://www.vagasemaraxa.com:5000/api/users/login")
        .body(serde_json::to_string(&LoginForm{email, password}).unwrap()) //format!("{{\"email\": \"{}\", \"password\":\"{}\"}}", email, password))
        .credentials(RequestCredentials::Include)
        .header("Content-Type", "application/json")
        .send().await.unwrap();

    if login_payload.status() == 401 {
        let error: ErrorResponse = login_payload.json().await.unwrap();
        return Err(error.message);
    } else {
        let data: User = login_payload.json().await.unwrap();
        return Ok(data);
    }
}

pub async fn get_user_info() -> Result<User, String> {
    let user_payload = Request::get("http://www.vagasemaraxa.com:5000/api/users/me")
        .credentials(RequestCredentials::Include)
        .send().await.unwrap();

    if user_payload.status() == 401 {
        let error: ErrorResponse = user_payload.json().await.unwrap();
        return Err(error.message);
    } else {
        let data: User = user_payload.json().await.unwrap();
        return Ok(data);
    }
}

pub async fn user_have_permission(permission_name: String) -> Result<bool, String> {
    let user_payload = Request::get(&format!("http://www.vagasemaraxa.com:5000/api/users/have-permission/{}", permission_name))
        .credentials(RequestCredentials::Include)
        .send().await.unwrap();

    #[derive(Serialize, Deserialize)]
    struct PermResponse {
        condition: bool
    }

    if user_payload.status() == 401 {
        let error: ErrorResponse = user_payload.json().await.unwrap();
        return Err(error.message);
    } else {
        let data: PermResponse = user_payload.json().await.unwrap();
        return Ok(data.condition);
    }
}