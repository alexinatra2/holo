# Holomorphic-Tinkering

Brief project description goes here.

---

## Prerequisites

### General Requirements
- **Rust**: You will need to have Rust installed on your machine to run this project.
    - Install Rust via the official [installation guide](https://www.rust-lang.org/tools/install).
    - For Linux:
      ```bash
      sudo apt install cargo
      ```
    - For Windows:  
      Download the Rust installer from the [official site](https://www.rust-lang.org/tools/install) and run it. During the installation, ensure the necessary tools (e.g., `cargo`, `rustc`) are added to your PATH.


- **Node.js**: To run the frontend application, you need a Node.js runtime installed. The easiest way to manage Node.js versions is by using **nvm**:
    - For Linux/Mac:  
      Follow the [nvm installation guide](https://github.com/nvm-sh/nvm).
    - For Windows:  
      Install [nvm-windows](https://github.com/coreybutler/nvm-windows/releases) and add it to your PATH. Then, install Node.js:
      ```bash
      nvm install Iron
      nvm use Iron
      ```

- **pnpm** (optional): For a faster Node.js package manager, install **pnpm**:
    - Universal installation:
      ```bash
      npm install --global pnpm
      ```
    - Install dependencies and run the frontend:
      ```bash
      pnpm -F react-frontend install
      pnpm run frontend
      ```

- **OpenCV**: Required for webcam functionalities.
    - For Linux:
      ```bash
      sudo apt update
      sudo apt install libopencv-dev pkg-config
      ```
    - For Windows:
        1. Download and install OpenCV from the [official website](https://opencv.org/releases/).
        2. Set the `OpenCV_DIR` environment variable to point to your OpenCV installation folder (e.g., `C:\opencv\build`).

---

## How to Run

### 1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/your-repo-name.git
   cd your-repo-name
   ```

### 2. Run the Program

- **For Linux/macOS**:
  ```bash
  cargo run
  ```

- **For Windows**:
  Open a terminal (PowerShell or Command Prompt) and run:
  ```bash
  cargo run
  ```

### 3. Enable More Options
Get additional information about available options:
```bash
cargo run --help 
# or
cargo run -h
```

---

## How to Install

To install this project as a binary called `holo`, follow these steps:

1. Navigate to the project root directory.
2. Run the following command:
   ```bash
   cargo install --path .
   ```

This will compile and install the binary to your system.

---

### Troubleshooting on Windows

- **Missing OpenCV libraries**: Ensure `OpenCV_DIR` is set correctly and that `pkg-config` is installed on your system.  
  Consider using the [vcpkg](https://github.com/microsoft/vcpkg) package manager to simplify OpenCV setup.

- **Path issues**: Ensure all necessary tools (e.g., `cargo`, `rustc`, `llvm-config`) are added to your system's PATH environment variable.

---