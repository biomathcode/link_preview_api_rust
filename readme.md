# Link Preview API

The Link Preview API is a Rust-based web service that fetches and provides metadata about a website when given its URL. It can extract information such as the website's title, description, favicon, and Open Graph image.

## Table of Contents

- [Link Preview API](#link-preview-api)
  - [Table of Contents](#table-of-contents)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
    - [Making API Requests](#making-api-requests)
  - [Contributing](#contributing)
  - [License](#license)

## Getting Started

### Prerequisites

Before you can use the Link Preview API, make sure you have the following prerequisites installed on your system:

- [Rust](https://www.rust-lang.org/tools/install): The Rust programming language and build tools.
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html): The Rust package manager.

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/biomathcode/link-preview-api.git


## Usage
### Running the Service
To run the Link Preview API service, use the following command:

```bash
cargo run --release
```

The service will start and listen on http://localhost:3000.




### Making API Requests
You can make API requests to the Link Preview service by sending a GET request to the root URL with a url query parameter. For example:

```bash
curl "http://localhost:3000/?url=https://example.com"
```


The API will respond with JSON data containing information about the website.

Here's an example of the JSON response:


```json
{
  "title": "Example Domain",
  "description": "This is an example domain used for illustrative examples in documents.",
  "favicon": "https://example.com/favicon.ico",
  "og_image": "https://example.com/images/logo.png"
}
```



## Contributing
Contributions are welcome! If you would like to contribute to the Link Preview API project, please follow these steps:

Fork the repository.
Create a new branch for your feature or bug fix: git checkout -b feature-name.
Make your changes and commit them: git commit -m "Add feature-name".
Push your changes to your fork: git push origin feature-name.
Create a pull request to the main repository.


## License
This project is licensed under the MIT License - see the LICENSE file for details.