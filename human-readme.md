# cyberlogs (a cli journal)

cyberlogs is a supersimple cli app, written in rust, that works for journaling on the cli.
basically you run it (´run.sh´), the program creates a txt file inside "journal/" with a date filename, and a quick title with date, and then it opens that text file with your text editor of choice. 

Yep it's pretty simple, but that's the whole idea, to make quick notes in a form of journal whenever you want to. i believe its a good way to keep track of your thoughts and stuff, whenever you are building a project, you call the program and start writing till it finishes, :wq and continue. Then whenever you got the time, you can read it all. 

## how to install

+ download source code:

´git clone https://github.com/casiojapi/cyberlogs.git´

+ build (with cargo):

´cargo build --release´

## how to use it

+ execute binary and open the file manually (boring way I)

´./target/release/cyberlogs´ -> output (ex: "journal/2023-05-05_1919.txt")

´vim journal/output.txt

+ use COMMANDS to make it fancier (in this case i use nvim as my text editor)

´./target/release/cyberlogs | xargs nvim´

+ or use the bash script (you can change your text editor inside "run.sh")

((give permissions to run.sh))

((´chmod +x .run.sh´))

- run the script (yes, like using an alias. ok) 

´./run.sh´



