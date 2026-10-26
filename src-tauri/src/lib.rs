use serde_json::json;

#[tauri::command]
async fn solve_task(question: String, image_base64: Option<String>) -> Result<String, String> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("API_KEY").map_err(|_| "Brak API_KEY w src-tauri/.env".to_string())?;
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "https://openrouter.ai/api/v1/chat/completions".to_string());
    let model = std::env::var("MODEL").unwrap_or_else(|_| "openai/gpt-4o-mini".to_string());

    let prompt = format!(r#"Jesteś pomocnym korepetytorem. Użytkownik wysłał zdjęcie zadania lub treść.

      TWOJE ZADANIE:
      1. Odczytaj treść zadania ze zdjęcia lub Dodatkowego pytania użytkownika.
      2. Jeśli masz widoczne odpowiedzi do pytania podawaj tylko suchą odpowiedź typu "2a". Nie wyjaśniaj jak do niej dojść. Jeśli jednak nie dostałeś możliwych odpowiedzi staraj się wutłumaczyć pytanie
      3. Jeśli na zdjęciu jest kilka zadań, odpowiedz na każde po kolei.
      Dodatkowe pytanie użytkownika: {}"#, question);

    let mut content = vec![json!({"type": "text", "text": prompt})];
    
    if let Some(img) = image_base64 {
        if !img.trim().is_empty() {
            content.push(json!({
                "type": "image_url",
                "image_url": { "url": img }
            }));
        }
    }

    let client = reqwest::Client::new();
    let res = client.post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "http://localhost")
        .header("X-Title", "stealth-app")
        .json(&json!({
            "model": model,
            "messages": [{ "role": "user", "content": content }]
        }))
        .send().await.map_err(|e| format!("Błąd połączenia: {}", e))?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(format!("API zwróciło błąd: {}", err_text));
    }

    let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    
    let answer = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Błąd odpowiedzi - brak content")
        .to_string();

    Ok(answer)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![solve_task])
    .run(tauri::generate_context!())
    .expect("error while running tauri app");
}