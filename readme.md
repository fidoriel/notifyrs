# notifyrs
WARNING: This is not ment to be hosted publicly for various reasons.

A very simple rust api where you can throw an json on and send an email. This is a micro service for your homeserver so you dont have to update passwords everywhere, just send notifications. There is ONE optional API Token, you may use it or not.

Test with Curl (note: you may update the target)
```bash
curl -X POST -H "Content-Type: application/json" -d '{"topic": "Hello, World!", "body": "This is the body of the email."}' http://0.0.0.0:8080/notify
```
With the `recipient_email` key in the post you can provide a list with recipients. If not set, it will default to the default the corresponding env var.

For use with the token append
```bash
-H "X-SECRET-KEY: 1234567890"
```
## dev
run: `cargo run`
format: `cargo fmt`
