# Get Started

Clone the repository:
```
git clone https://github.com/DSS3113/CS128H-Project-The-Indomitable-Dominators.git
cd CS128H-Project-The-Indomitable-Dominators
```

Then follow the install instructions for torch from [tch-rs](https://github.com/LaurentMazare/tch-rs#getting-started).
Ensure you have a compatible version of cuda for best performance.

Download the [weights](https://drive.google.com/file/d/1KxgrUkgC3TeRWmW8GEmf9QWU4n5-KCpU/view?usp=sharing) to the root project directory.

Start the server: `cargo run` (may take some time if building torch from source)

Open `ui.html` in a web browser to select and upload the target images, enter the output email, and click "Submit".