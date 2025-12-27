# imgur
web api for imgur.com Find, rate and share the best memes and images. Discover the magic of the Internet at Imgur.
# main
```rust
mod imgur;
use imgur::Imgur;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let imgur_client = Imgur::new();
    match imgur_client.get_post_meta("RUjYvXN","post,user,accolades").await {
        Ok(json) => {
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        Err(e) => {
            eprintln!("erorr: {}", e);
        }
    }

    Ok(())
}
```

# Launch (your script)
```
cargo run
```
