# Metered API Server

This is a metered API server written in Rust using the `axum` web framework. It provides a simple and effective way to rate-limit API requests on a per-user and per-route basis.

## Features

-   **Per-User Rate Limiting:** Each user is identified by an API token.
-   **Per-Route Rate Limiting:** Each of the three API routes (`/x`, `/y`, `/z`) is metered individually for each user.
-   **Tiered Rate Limits:** The rate limits are configurable for requests per minute, per hour, and per day.
-   **Modern Rust:** Built with modern Rust libraries like `axum`, `tokio`, and `dashmap`.

## Getting Started

### Prerequisites

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
-   `curl` for testing

### Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/metered-api-server.git
    cd metered-api-server
    ```

2.  **Configure the rate limits:**
    Create a `.env` file in the root of the project and add the following content:
    ```
    # Rate Limiting Configuration
    REQUESTS_PER_MINUTE=5
    REQUESTS_PER_HOUR=100
    REQUESTS_PER_DAY=1000

    # Server Configuration
    SERVER_ADDR=127.0.0.1:3000
    ```

### Running the Server

To start the server, run the following command:

```bash
cargo run
```

The server will start and listen on the address specified in the `.env` file (e.g., `127.0.0.1:3000`).

## Testing the API

### Generate a Test API Token

You can use a simple tool like `uuidgen` to generate a test API token:

```bash
uuidgen
```

### Making API Requests

To make requests to the API, you need to include the API token in the `Authorization` header as a bearer token.

Here are some `curl` commands to test the API routes:

```bash
# Replace <YOUR_API_TOKEN> with the token you generated.
export API_TOKEN=<YOUR_API_TOKEN>

# Test the /x route
curl -H "Authorization: Bearer $API_TOKEN" http://127.0.0.1:3000/x

# Test the /y route
curl -H "Authorization: Bearer $API_TOKEN" http://127.0.0.1:3000/y

# Test the /z route
curl -H "Authorization: Bearer $API_TOKEN" http://127.0.0.1:3000/z
```

### Testing the Rate Limiting

To test the rate limiting, you can use a simple loop in your shell. For example, to test the per-minute limit on the `/x` route:

```bash
for i in {1..10}; do
  curl -H "Authorization: Bearer $API_TOKEN" http://127.0.0.1:3000/x
  sleep 1
done
```

You should see the first 5 requests succeed, and the subsequent requests fail with a `429 Too Many Requests` error.
