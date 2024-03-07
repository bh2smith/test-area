
On macOS:

```sh
cargo run
```
results in Reqwest::Error ("bad protocol version").

However, in Docker:

```sh
docker build -t test-area .
docker run -rm test-area
```
Fetches the content at: https://metroverse.com/blocks/14623

What does it take to make this run outside of a docker image?
