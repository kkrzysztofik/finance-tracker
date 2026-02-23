use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Deserialize)]
struct CategoryAssignment {
    index: usize,
    category: String,
}

pub struct CategorizeResult {
    pub total: i32,
    pub categorized: i32,
    pub failed: i32,
}

pub async fn categorize_uncategorized(
    pool: &PgPool,
    api_key: &str,
) -> Result<CategorizeResult, String> {
    let client = Client::new();

    // Fetch uncategorized transactions
    let uncategorized: Vec<(i32, String, Option<String>, String)> = sqlx::query_as(
        "SELECT id, description, counterparty, amount::text \
         FROM transactions \
         WHERE category_id IS NULL \
         ORDER BY transaction_date DESC \
         LIMIT 200",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| e.to_string())?;

    if uncategorized.is_empty() {
        return Ok(CategorizeResult {
            total: 0,
            categorized: 0,
            failed: 0,
        });
    }

    // Fetch categories
    let categories: Vec<(i32, String)> =
        sqlx::query_as("SELECT id, name FROM categories ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?;

    let category_list: String = categories
        .iter()
        .map(|(_, name)| name.as_str())
        .collect::<Vec<_>>()
        .join(", ");

    let total = uncategorized.len() as i32;
    let mut categorized = 0i32;
    let mut failed = 0i32;

    // Process in batches of 50
    for batch in uncategorized.chunks(50) {
        let transactions_text: String = batch
            .iter()
            .enumerate()
            .map(|(i, (_, desc, counterparty, amount))| {
                let cp = counterparty.as_deref().unwrap_or("Unknown");
                format!("{}: {} | {} | {}", i, desc, cp, amount)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let system_prompt = format!(
            "You are a financial transaction categorizer. \
             Categorize each transaction into exactly one of these categories: {}.\n\n\
             Respond with a JSON object containing a \"results\" key with an array of objects, \
             each with 'index' (number) and 'category' (string matching one of the categories exactly).\n\
             Example: {{\"results\": [{{\"index\": 0, \"category\": \"Groceries\"}}]}}",
            category_list
        );

        let user_prompt = format!("Categorize these transactions:\n{}", transactions_text);

        let request = ChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                Message {
                    role: "system".to_string(),
                    content: system_prompt,
                },
                Message {
                    role: "user".to_string(),
                    content: user_prompt,
                },
            ],
            response_format: ResponseFormat {
                type_: "json_object".to_string(),
            },
        };

        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("OpenAI request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            failed += batch.len() as i32;
            info!("OpenAI error ({}): {}", status, body);
            continue;
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

        let content = &chat_response.choices[0].message.content;

        // Parse the response - it might be {"results": [...]} or just [...]
        let assignments: Vec<CategoryAssignment> =
            if let Ok(arr) = serde_json::from_str::<Vec<CategoryAssignment>>(content) {
                arr
            } else if let Ok(wrapper) = serde_json::from_str::<serde_json::Value>(content) {
                if let Some(arr) = wrapper.as_object().and_then(|o| o.values().next()) {
                    serde_json::from_value(arr.clone()).unwrap_or_default()
                } else {
                    vec![]
                }
            } else {
                info!("Failed to parse categorization response: {}", content);
                failed += batch.len() as i32;
                continue;
            };

        // Update transactions
        for assignment in &assignments {
            if assignment.index >= batch.len() {
                continue;
            }

            let tx_id = batch[assignment.index].0;

            // Find category ID by name
            if let Some((cat_id, _)) =
                categories.iter().find(|(_, name)| name == &assignment.category)
            {
                sqlx::query(
                    "UPDATE transactions SET category_id = $1, category_source = 'ai' \
                     WHERE id = $2 AND category_id IS NULL",
                )
                .bind(cat_id)
                .bind(tx_id)
                .execute(pool)
                .await
                .map_err(|e| format!("Update error: {}", e))?;

                categorized += 1;
            } else {
                failed += 1;
            }
        }

        let batch_unmatched = batch.len() as i32 - assignments.len() as i32;
        if batch_unmatched > 0 {
            failed += batch_unmatched;
        }
    }

    info!(
        "Categorization complete: {total} total, {categorized} categorized, {failed} failed"
    );
    Ok(CategorizeResult {
        total,
        categorized,
        failed,
    })
}
