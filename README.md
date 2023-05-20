# cyberlogs: cli journaling app 

![cieguito](https://pbs.twimg.com/profile_banners/3438631625/1681945275/1500x500)

<br>
With just a few keystrokes, you can create a journal entry, add your thoughts, and save them in the "journal/" directory, like whispers of your soul echoing in the void of cyberspace. The lightweight nature of this app makes it the perfect tool for tracking your ideas, progress, and memories as you wander through the dystopian landscape of your projects.

## Features
- Automatically generates a date-based text file, like a beacon of light in the darkness.
- Quick title generation with the current date, marking your place in the timeline.
- Opens the created text file in your favorite text editor.
- Organizes your journal entries in the "journal/" directory, a sanctuary for your thoughts.
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

### Option 1: execute the binary and open the file manually (boring way)

1. Run the binary:


```bash
./target/release/cyberlogs
```

2. The output will reveal the path to the created file, e.g. "journal/2023-05-05_1919.txt". Open it using your text editor of choice:

```bash
vim journal/2023-05-05_1919.txt
```

### Option 2: use commands to make it fancier (recommended)

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

A cool way to use it from any place (so you don't have to cd into the directory every time you want to journal) is to add an alias to your zsh or bash config file. This way you can just type "cyberlogs" (or whatever you want to call the alias) from any directory log whatever you want to log save the file and continue doing what you've been doing. I'll be doing the example using zsh, but if you use bash (it will be .bashrc) or any other than zsh, just look on how is your config file name and follow the steps, it should be the same logic.

1. Open your zsh configuration file with your preferred text editor. For example, if you use nano, you can use the following command:

```bash
nano ~/.zshrc
```

2. At the end of this file, add a new alias that points to the run script. For instance, if you want to use the alias cyberlogs, you could add the following line:

```bash
alias cyberlogs='/Users/casiojapi/Documents/repos/cyberlogs/run.sh'
```

3. Save your changes and exit the text editor. If you're using nano, you can do this by pressing Ctrl+X, then Y to confirm that you want to save the changes, and finally Enter to confirm the file name.

4. Source your zsh configuration file to apply the changes immediately. You can do this with the following command:
 
 ```bash                                                             
source ~/.zshrc
 ```

5. Log using your new alias, anytime anywhere.

## Contributing

If you have questions, suggestions, or ideas for improvement, open an issue or join our discussion. Make cyberlogs the pinnacle of edgy and arrogant cli journaling

hit me up on twitter: [@casiojapi](https://twitter.com/casiojapi)

