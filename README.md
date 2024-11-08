# holomorphic-tinkering

Brief project description goes here.

## Prerequisites

- **Rust**: You will need to have Rust installed on your machine to run this project.
  
  You can install Rust by following the official [installation guide](https://www.rust-lang.org/tools/install) or simply install cargo using `sudo apt install cargo`

- **Node**: To run the frontend application you need to have a node runtime installed. Setting this
up is easiest accomplished using [nvm](`https://github.com/nvm-sh/nvm`). Once installed, ensure
`npm` is installed using the below commands:

```bash
nvm install Iron
nvm use Iron
```
- **pnpm** (optional): To run using a faster (Rust-based) node package manager, install `pnpm`
using the following commands:

```bash
npm install --global pnpm
pnpm -F react-frontend install
pnpm run frontend
```
- **opencv**: for the webcam to work you will need to install the required system libraries:

```bash
sudo apt update
sudo apt install libopencv-dev pkg-config
```

## How to Run

Once you have Rust installed, follow these steps to run the project:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/your-repo-name.git
   ```

2. **Put an image file into the `./src/images/input/` and call it for example `test.jpg`**

3. **Run the program (path = filename in input dic):**

**Normal polynom f(z) = 0 + 1*z^1 + 3*z^2**
  ```bash
  cargo run test.jpg 0 1 3
  ```

**Fractional rational function f(z) = (3z + 2z^2 + 3) / (3*z^4)**
  ```bash
  cargo run test.jpg 3z+2z^2+3/3*z^4
  ```

4. **The output image will be saved to `./output/test_0.0_1.0_3.0.jpeg`**

