# Discord Webhooks

This is a simple library for interacting with [Discord's webhooks](https://discord.com/developers/docs/resources/webhook).

### Examples

```rust
use discord-webhooks::Webhook;
fn execute() {
    let hook = Webhook::new(id, token);
    Webhook::execute_webhook(hook, "{content: 'hi', embeds: [], tts = false }").unwrap();
}
fn delete() {
    Webhook::delete_Webhook(hook).unwrap();
}
fn get() {
    Webhook::get_webhook(hook).unwrap();
}
fn edit_message() {
    Webhook::edit_message(hook, body, message_id)
}
```
