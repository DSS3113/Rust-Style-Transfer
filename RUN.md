# Get Started

Clone the repository:
```
git clone https://github.com/DSS3113/CS128H-Project-The-Indomitable-Dominators.git
cd CS128H-Project-The-Indomitable-Dominators
```

Then follow the install instructions for torch from [tch-rs](https://github.com/LaurentMazare/tch-rs#getting-started).
Ensure you have a compatible version of cuda for best performance.

Download the [weights](https://drive.google.com/file/d/1KxgrUkgC3TeRWmW8GEmf9QWU4n5-KCpU/view?usp=sharing) to the root project directory.

Create a `.env` file in the following format:
```
FROM_GMAIL_ADDRESS="gmailaddress@gmail.com"
GMAIL_UNIQUE_PASSWORD="uniquepassword"
```
These are the credentials to the Gmail address that will be used for sending the output image to the specified email address. Please keep in mind that you must specify a Gmail address as other email providers don't work for our purposes.

If you have an M1 Mac, please follow the following the additional steps given below:
- install miniforge with homebrew -- See https://naolin.medium.com/conda-on-m1-mac-with-miniforge-bbc4e3924f2b
- create a new conda environment: `conda env create -f environment.yml`
- activate the new environment: `conda activate proj-env`
- create a symlink in this repo: `ln -sf /opt/homebrew/Caskroom/miniforge/base/envs/proj-env/lib/python3.9/site-packages/torch/ torch`

Start the server: `cargo run` (may take some time if building torch from source)

Open `ui.html` in a web browser to select and upload the target images, enter the output email, and click "Submit".
