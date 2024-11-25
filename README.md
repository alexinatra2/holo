# Holomorphic-Tinkering

Dieses Projekt kombiniert Rust und WebAssembly, um Bildverarbeitungsalgorithmen im Browser auszuführen. Es verwendet eine Rust-Bibliothek, die in WebAssembly kompiliert wird, um Bilder zu transformieren und zu manipulieren. Die Frontend-Anwendung ist in React geschrieben und ermöglicht es Benutzern, Bilder hochzuladen und zu transformieren oder die Transformationen in Echtzeit zu sehen via Webcam.

Das Projekt nutzt holomorphe Funktionen, um komplexe Bildtransformationen durchzuführen. Holomorphe Funktionen sind in der komplexen Analysis von Bedeutung und ermöglichen es, glatte und kontinuierliche Transformationen auf Bildern anzuwenden, was zu beeindruckenden visuellen Effekten führt.

---

This project combines Rust and WebAssembly to execute image processing algorithms in the browser. It uses a Rust library compiled to WebAssembly to transform and manipulate images. The frontend application is written in React and allows users to upload images and to transform or see the transformations in real-time via webdam.

The project utilizes holomorphic functions to perform complex image transformations. Holomorphic functions are significant in complex analysis and allow for smooth and continuous transformations on images, resulting in impressive visual effects.

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
        1. Navigate to the project root directory.
        2. Run the following command:
      ```bash
      pnpm -F react-frontend install
      pnpm run frontend
      ```
      3. Troubleshooting
      ```bash
         cargo install wasm-pack
         wasm-pack build --target web --out-dir pkg

      ```

- **OpenCV**: Required for webcam functionalities.
    - For Linux:
      ```bash
      sudo apt update
      sudo apt install libopencv-dev pkg-config
      ```
      On Linux, if this fails due to missing packages, consider installing OpenCV from the source:
      ```bash
      sudo apt update
      sudo apt install build-essential cmake git pkg-config libgtk-3-dev libavcodec-dev \
      libavformat-dev libswscale-dev libv4l-dev libxvidcore-dev libx264-dev libjpeg-dev \
      libpng-dev libtiff-dev gfortran openexr libatlas-base-dev python3-dev python3-numpy \
      libtbb2 libtbb-dev libdc1394-22-dev
      git clone https://github.com/opencv/opencv.git
      git clone https://github.com/opencv/opencv_contrib.git
      cd opencv && mkdir build && cd build
      cmake -D CMAKE_BUILD_TYPE=Release -D CMAKE_INSTALL_PREFIX=/usr/local \
      -D OPENCV_EXTRA_MODULES_PATH=../../opencv_contrib/modules ..
      make -j$(nproc)
      sudo make install
      sudo ldconfig
      ```
    - For Windows:
        1. Download and install OpenCV from the [official website](https://opencv.org/releases/).
        2. Set the `OpenCV_DIR` environment variable to point to your OpenCV installation folder (e.g., `C:\opencv\build`).

---
## How to Run

### 1. Clone the repository
   ```bash
   git clone https://github.com/alexinatra2/holomorphic-tinkering.git
   cd holomorphic-tinkering
   ```

### 2. Run the Program
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

