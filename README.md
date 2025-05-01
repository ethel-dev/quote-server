# Quote Server
## Ethel Arterberry for Rust Full-Stack Web at PSU

## Overview

This project implements a basic server-side website using Axum and Askama to display a single quote. Basic templating is included and not much else.

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

The application will display a single quote or recipe on the main page. One can modify the content by updating the data structures in `src/models.rs` and the templates in the `templates` directory.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.