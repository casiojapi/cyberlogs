# cyberlogs: cli journaling app 

![cieguito](https://pbs.twimg.com/profile_banners/3438631625/1681945275/1500x500)

<br>
With just a few keystrokes, you can create a journal entry, add your thoughts, and save them in the "journal/" directory, like whispers of your soul echoing in the void of cyberspace. The lightweight nature of this app makes it the perfect tool for tracking your ideas, progress, and memories as you wander through the dystopian landscape of your projects.

## Features
- Automatically generates a date-based text file.
- Opens the created text file in your favorite text editor.
- Organizes your journal entries in the "journal/" directory.
- Customizable text editor in the provided run.sh script.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/casiojapi/cyberlogs.git
```

2. Build the project using cargo:

```bash
cd cyberlogs
cargo build --release
```

## Usage

### Option 1: execute the binary and open the file manually 

1. Run the binary:

```bash
./target/release/cyberlogs
```

2. The output will reveal the path to the created file, e.g. "journal/2023-05-05_1919.txt". Open it using your text editor of choice:

```bash
vim journal/2023-05-05_1919.txt
```

### Option 2:

1. In this example, we'll use neovim as our text editor:

```bash
./target/release/cyberlogs | xargs nvim
```

### Option 3: use the provided bash script (most convenient)

1. First, grant execution permissions to the run.sh script:

```bash
chmod +x run.sh
```

2. Modify the `run.sh` script to change the text editor, if desired.

3. Run the script:

```bash
./run.sh
```

## Use it anywhere

```bash
alias cyberlogs='/path/to/cyberlogs/run.sh'
```
## Contributing

If you have questions, suggestions, or ideas for improvement, open an issue.

