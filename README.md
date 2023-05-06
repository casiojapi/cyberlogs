# Cyberlogs: A Dystopian CLI Journal from Argentina

![cyberlogs](https://i.imgur.com/1IHxh3b.png)

In the digital wastelands of a post-apocalyptic cyberpunk world, one lone Argentinian developer brings you Cyberlogs - a super simple CLI app, forged in the Rust programming language, to help you navigate the chaotic realm of journaling on the command line.

With just a few keystrokes, you can create a journal entry, add your thoughts, and save them in the "journal/" directory, like whispers of your soul echoing in the void of cyberspace. The lightweight nature of this app makes it the perfect tool for tracking your ideas, progress, and memories as you wander through the dystopian landscape of your projects.

## 🌆 Features
- Automatically generates a date-based text file, like a beacon of light in the darkness
- Quick title generation with the current date, marking your place in the timeline
- Opens the created text file in your favorite text editor, the weapon of choice for code warriors
- Organizes your journal entries in the "journal/" directory, a sanctuary for your thoughts
- Customizable text editor in the provided run.sh script

## 📡 Installation

1. Clone the repository, like connecting to the Matrix:

```bash
git clone https://github.com/casiojapi/cyberlogs.git
```

2. Build the project using Cargo, the fuel for your cyber-engine:

```bash
cd cyberlogs
cargo build --release
```

## 📚 Usage

### Option 1: Execute the binary and open the file manually (boring way)

1. Run the binary, like activating a cybernetic implant:

```bash
./target/release/cyberlogs
```

2. The output will reveal the path to the created file, e.g. "journal/2023-05-05_1919.txt". Open it using your text editor of choice, the key to unlock your thoughts:

```bash
vim journal/2023-05-05_1919.txt
```

### Option 2: Use commands to make it fancier (recommended)

1. In this example, we'll use NeoVim as our text editor, the hacker's ultimate tool:

```bash
./target/release/cyberlogs | xargs nvim
```

### Option 3: Use the provided bash script (most convenient)

1. First, grant execution permissions to the run.sh script, like hacking into a mainframe:

```bash
chmod +x run.sh
```

2. Modify the `run.sh` script to change the text editor, if desired.

3. Run the script, like launching a cyber-attack:

```bash
./run.sh
```

## 🤝 Contributing

Cyberlogs is an open-source project from the dystopian depths of Argentina, and we welcome rebels and renegades who wish to contribute to this digital revolution! Feel free to fork the repository, make your changes, and submit a pull request. If you have questions, suggestions, or ideas for improvement, please open an issue or join our discussion. Together, we can forge a new path in the cyberpunk realm of CLI journaling!
