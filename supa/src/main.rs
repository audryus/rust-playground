use futures::executor::block_on;
use supabase::SupabaseClient;

fn main() {
    println!("Hello, world!");
    let supabase_url = "SUPA_URL";
    let supabase_key = "SUPA_KEY";
    let client = SupabaseClient::new(supabase_url, supabase_key);

    block_on(sign_in_with_password(client));
}
async fn sign_in_with_password(client: SupabaseClient) {
    let response = client
        .auth
        .sign_in_with_password("user@example.com", "strong_password")
        .await;

    match response {
        Ok(session) => println!("User signed in with session: {:?}", session),
        Err(error) => println!("Sign in error: {:?}", error),
    }
}
