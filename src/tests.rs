use super::Webhook;
#[cfg(test)]
#[test]
#[ignore]

fn get_webhook() {
    Webhook::get_webhook(
        Webhook::new(
            &std::env::var("dischook_id").unwrap(),
            &std::env::var("dischook_token").unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn get_webhook_invalid() {
    Webhook::get_webhook(Webhook::new("fake", "fake").unwrap()).unwrap();
}

#[test]
#[ignore]
fn delete_webhook() {
    Webhook::delete_webhook(
        Webhook::new(
            &std::env::var("dischook_id").unwrap(),
            &std::env::var("dischook_token").unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
}

#[test]
#[should_panic]
fn delete_webhook_invalid() {
    Webhook::delete_webhook(Webhook::new("fake", "fake").unwrap()).unwrap();
}
