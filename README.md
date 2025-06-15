# Quote Server
## Ethel Arterberry for Rust Full-Stack Web at PSU

## Overview

This project implements a basic server-side website using Axum, Askama and SQLite to serve quotes to the user. A basic Utopia-documented REST API is implemented for CRUD and has Swagger documentation.

## Setup Instructions

To set up the project, follow these steps:

1. Clone the repository:
```sh
git clone <repository-url>
cd quote-server
```

2. Build the project:
```sh
cargo build
```

3. Run the application:
```sh
cargo run
```

4. Open your web browser and navigate to `http://localhost:3000` to view the application.

## Usage

You can add a quote by making a POST request to the `/quotes/` route.

```sh
curl -X POST http://localhost:3000/quotes \
  -H "Content-Type: application/json" \
  -d '{
    "text": "The only thing we have to fear is fear itself.",
    "author": "Franklin D. Roosevelt", 
    "source": "Speech",
    "tags": ["motivation", "determination"]
  }'
```

Searching is also possible by making a GET request to the `/quotes/search` route with `tag` or `author` or just a search term.

```sh
curl "http://localhost:3000/quotes/search?author=Roosevelt"
curl "http://localhost:3000/quotes/search?tag=motivation"
curl "http://localhost:3000/quotes/search?search=determination"
```

## License

This project is licensed under the MIT License. See the LICENSE file for more details.