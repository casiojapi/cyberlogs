# cyberlogs: a dystopian cli journal from argentina

with just a few keystrokes, you can create a journal entry, add your thoughts, and save them in the "journal/" directory, like whispers of your soul echoing in the void of cyberspace. the lightweight nature of this app makes it the perfect tool for tracking your ideas, progress, and memories as you wander through the dystopian landscape of your projects.

## features
- automatically generates a date-based text file, like a beacon of light in the darkness.
- quick title generation with the current date, marking your place in the timeline.
- opens the created text file in your favorite text editor.
- organizes your journal entries in the "journal/" directory, a sanctuary for your thoughts.
- customizable text editor in the provided run.sh script.

## installation

1. clone the repository:

```bash
git clone https://github.com/casiojapi/cyberlogs.git
```

2. build the project using cargo:

```bash
cd cyberlogs
cargo build --release
```

## usage

### option 1: execute the binary and open the file manually (boring way)

1. run the binary:


```bash
./target/release/cyberlogs
```

2. the output will reveal the path to the created file, e.g. "journal/2023-05-05_1919.txt". open it using your text editor of choice:

```bash
vim journal/2023-05-05_1919.txt
```

### option 2: use commands to make it fancier (recommended)

1. in this example, we'll use neovim as our text editor:

```bash
./target/release/cyberlogs | xargs nvim
```

### option 3: use the provided bash script (most convenient)

1. first, grant execution permissions to the run.sh script:

```bash
chmod +x run.sh
```

2. modify the `run.sh` script to change the text editor, if desired.

3. run the script:

```bash
./run.sh
```

## contributing

if you have questions, suggestions, or ideas for improvement, open an issue or join our discussion. make cyberlogs the pinnacle of edgy and arrogant cli journaling
