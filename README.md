# ICP Backend Template in Rust

This guide includes basic stable structure implementation, along with implementation of pagination, automated DID file generation, test scripts, and guards.

## Getting Started

To use this template, follow the steps below:

### 1. Clone the Repository

```bash
git clone https://github.com/Rohanarora17/icp_backend_template_in_rust.git
cd icp_backend_template_in_rust
```

### 2. start dfx
```bash
dfx start --background
```

### 3. Generate the DID File
```bash
./scripts/generate_candid.sh
```
### 4. Deploy the Project Locally 
```bash
dfx deploy
```

### 5. Test Functions with Dummy Data
```bash
./tests/user.sh
```

## Project Structure

- `src/`: Contains the source code for the project.
- `scripts/`: Contains scripts for generating DID files and other automation tasks.
- `tests/`: Contains test scripts to validate your implementation.

## Features

- Basic stable structure implementation.
- Pagination implementation.
- Automated DID file generation.
- Test scripts to create dummy data and validate functions.
- Guards for added security and data integrity.

## Usage

- Modify the source code in the `src/` directory to suit your needs.
- Use the provided scripts for automation tasks.
- Run the test scripts to ensure your implementation is correct.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request for any improvements or bug fixes.

## License

This project is licensed under the MIT License.
