# holomorphic-tinkering

Brief project description goes here.

## Prerequisites

- **Rust**: You will need to have Rust installed on your machine to run this project.
  
  You can install Rust by following the official [installation guide](https://www.rust-lang.org/tools/install) or simply install cargo using `sudo apt install cargo`

## How to Run

Once you have Rust installed, follow these steps to run the project:

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/your-repo-name.git
   ```

2. **Put an image file into the `./src/images/input/` and call it for example `test.jpg`**

3. **Run the program (path = filename in input dic):**

  ```bash
  cargo run test.jpg 0 1 3
  ```

4. **The output image will be saved to `./output/test_0.0_1.0_3.0.jpeg`**

