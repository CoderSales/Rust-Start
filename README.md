# Rust-Start

## Instructions assuming some knowledge of GitHub but otherwise from scratch:

## to clone

click on `Code` on GitHub Repository

click `Copy url` button on right of drop down that appears

## on Windows

go to VSCode

Pick a (local) folder location

<kbd>Ctrl</kbd><kbd>'</kbd'>

to make terminal appear

ensure using git bash (otherwise install here:) [git](https://git-scm.com/downloads)

____

### can skip next part if git bash appears in VSCode terminal under little "v" at top right of pop up terminal in VSCode

may need WSL: (Windows Subsystem for Linux): here: [install WSL](https://learn.microsoft.com/en-us/windows/wsl/install)

- if so, Copy PowerShell command from page or from here: `wsl --install`

    - open PowerShell (or install from:) | [Install PowerShell using Winget (recommended)](https://learn.microsoft.com/en-us/powershell/scripting/install/installing-powershell-on-windows?view=powershell-7.4#install-powershell-using-winget-recommended) | or | here: | `winget search Microsoft.PowerShell` | pasting command into PowerShell using winget (which seems to assume that you already have PowerShell however, there is also an option to install .msi from page but the winget instruction given earlier is recommended) | Also can press <kbd>Windows</kbd> and type `Microsoft Store` wait for it to open and Search for PowerShell and install fro there.

    - once PowerShell is installed (may need to add to PATH variable, either System or User version (Note if open User only then may not be able to access system PATH but if press <kbd>Windows</kbd> Search `system environment variables` at top of Windows Search bar, will be able to access both System and User PATH variables) --> `Environment Variables` --> There are 2 boxes ontop of each other, the top box has User Variables, The bottom has System variables, System variables are the more general case, allowing all users access to programs whose PATH variable is set here. So, in either User or System box (say System box): find Path: Click, Edit Click New paste previously copied PATH to relevant app's .exe file (which can be found by going: <kbd>Windows</kbd> typing name of app (say `PowerShell`) and right clicking on the correct app that should come up if installed, otherwise just get internet search results, but assuming it is installed, right click, then click on `open file location` if folder lonly has shortcuts, click on the relevant shortcut (pick one if more than one shortcut) click `open file location` (if selected this makes it easier, but in any case:) right click on the relevant file (of type Application) and click `Copy as path`))

        - once PowerShell is installed:

            - and WSL is installed:

                - can install the git bash as mentioned above, or can skip these bulletpoint and indented steps if git bash just installs on its own.

So now, assuming you have git bash installed 

and assuming you have VSCode installed (else install from here:) [VSCode](https://code.visualstudio.com/)

Then need to ensure VSCode "knows" that git bash is installed (Pay attention to options during installation, although most defaults are fine): 

Assuming VSCode knows about git bash (if just installed restart VSCode) (may even need to Shutdown and Turn on computer if just installed some / all of these applications): 

- may need to add `git bash` to PATH variable (so add the path to the .exe file for this application, by finding it right clicking on it and selecting copy PATH and pasting the copied PATH into the PATH variable section referred to previously )

____

<kbd>Ctrl</kbd><kbd>'</kbd'>

to make terminal appear

- Then Click on: little "v" at top right of the terminal that appears (may need to press <kbd>Ctrl</kbd><kbd>'</kbd'> a few times, to get comfortable with toggling terminal on / off (sometimes instead of <kbd>'</kbd> it can be say <kbd>\`</kbd> "backtick" instead of <kbd>'</kbd> "apostrophe", depending on keyboard type, (or if not using Windows, but can Google Keyboard Shortcut then, or just go to the bar at the top of VSCode (which, if it has disappeared pressing <kbd>F11</kbd> should make it reappear) and in bar at top read from top left: File, Edit, Seleection, View, Go, Run, Terminal, Help and Select Terminal | Then `New Terminal` ) Then ) <kbd>Ctrl</kbd><kbd>'</kbd'>
should allow to hide Terminal or can click "x" at top right of Terminal Window in VScode).

then type:

git clone `paste previously copied url from GitHub` (from start of document (or recopy per above, earlier step, if have not got in paste memory (also, can try <kbd>Windows</kbd>v</kbd> to look at "clipboard (may need to turn on enable clipboard history going forward here if not done already))))

and type <kbd>Enter</kbd>

to "clone" GitHub Repository from remote GitHub to local machine

Then Go to: File > Open Folder

and go to the folder that you were already at (which should just open to it here automatically) and should see the newly cloned folder, so double click into this folder, check that say the .git folder is not accidentally selected and does nto autopopulate into the File box at bottom of open window, if it does, or in any event , make sure to clear File name bar at bottom of window once into Rust-Start folder cloned.

Then Click Open

This should open a New VSCode Window (which you can always do by going: File > New Window) and here if you <kbd>Ctrl</kbd><kbd>'</kbd> to bring up terminal window in VScode at bottom of screen, and now if you select the "v" at top right, it should give option to select the git bash terminal (if not selected by default, as sometimes PowerShell (or another CLI (Command Line Interface) is the default)) should open, and so now you can:

(assuming Rust already installed or else go to : 27-Tutorial.md below and click first link to programiz tutorial and follow instructions there), otherwise, if rust is installed (check by going <kbd>Windows</kbd> and typing `Command Prompt` and pressing <kbd>Enter</kbd> to open a Command Prompt Window (independently of VSCode (or can use VSCode CLI) and can type rustc --version to check if Rust is installed, and if get rustc followed by numbers (numbers and letters and a date) then rustc is installed and PATH is correctly configured.))

#### how to disconnect from remote if cloning from GitHub repository:

per [How to Remove a Git Remote](https://www.linode.com/docs/guides/git-remote-remove-origin/#how-to-remove-a-git-remote):

```bash
git remote -v
```

to list remotes

and

```bash
git remote rm origin
```

to remove the original remote from which the repository was cloned

Then can scrolling back up the reference provided above, or just clicking here on: [What is a Git Remote?](https://www.linode.com/docs/guides/git-remote-remove-origin/#what-is-a-git-remote), can add your own origin:

(Note this assumes you have setup a GitHub account, which you can just do or Google how to):

then (**substituing for user with your GitHub username** as well as **your GitHub Repository**, which you will also need to make after having setup GitHub, and then copied the url to *your* GitHub Repository), you can: add an origin: using your particular variant of this "kind" of command:

```bash
git remote add origin https://github.com/user/repo.git
```

(basically a `git` `remote` `add` `origin` command with your `**GitHub-username**` whichever one of your `**GitHub-repository-name**`s inserted into the url like https://github.com/`username`/`repositoryName` (without backticks if they are visible, ignore this comment if not) )

will add your remote repo to your VSCode local environment so that they "talk to" and are "aware of" each other.

now `git status` commands will work as well as standard commands to `git add .` to add (or stage) everything, like packing luggage to use a phyucial analogy, and `git commit -m "Any message under 50 characters preferably"` to "label your luggage and put it on a pallet near the door, and `git pull origin main` to "check if there are any incoming files (to avoid git merge issues often avoided by just doing the aforementioned git pull)" and `git push origin main` (by analogy becuase otherwise it can be difficult to understand the 3 or 4 step process of "saving to remote" the first time): to push the pallet off into a van in the driveway or a boat into a lake where it then sails or is shipped to the remote repository in the cloud (on GitHub).

If there are errors, these may be to do with setting up your `git username` and `git email` locally, which you can either refer here for: [gitlab - "Make sure you configure your 'user.email' and 'user.name' in git" when trying to push to git lab - Stack Overflow](https://stackoverflow.com/questions/54876421/make-sure-you-configure-your-user-email-and-user-name-in-git-when-trying-t) or: can just copy *and then modify slightly* these 2 commands from the last link: 1. `git config --global user.name "John Doe"` 2. `git config --global user.email "johndoe@email.com"` substituting *your* git username and email (to match GitHub username and email), and then this should resolve any errors like: [`Make sure you configure your 'user.email' and 'user.name' in git".`](https://stackoverflow.com/questions/54876421/make-sure-you-configure-your-user-email-and-user-name-in-git-when-trying-t)

and per above reference can check all setup for git usage by running `git remote -v`.

After that all works, continue.

So, then can:

____

## once above basic setup (or other similar or equivalent setup) is done:

Scroll to bottom of `README` or `Ctrl F` for 

`notes on running`

if returning and just want basic commands

## Preliminary

**have a quick look at instructions here:** [Start.md](/documentation/01-Start.md)

**just a note on a bug [Fixed]** [Cargo.md](/documentation/02-cargoBook.md)

(uses cargo commands): [Commands.md](/documentation/03-Commands.md)

## Programiz Tutorial Progress:

**Start here:** [27-Tutorial.md](/documentation/27-Tutorial.md)

[28-Tutorial-Tuple-onwards.md](/documentation/28-Tutorial-Tuple-onwards.md)

[29-Tutorial-Functions-onwards.md](/documentation/29-Tutorial-Functions-onwards.md)

[30-Tutorial-Closure-onwards.md](/documentation/30-Tutorial-Closure-onwards.md)

[31-Tutorial-Vector-onwards.md](/documentation/31-Tutorial-Vector-onwards.md)

[32-Tutorial-Rust-String-onwards.md](/documentation/32-Tutorial-Rust-String-onwards.md)

[33-Tutorial-HashMap-onwards.md](/documentation/33-Tutorial-HashMap-onwards.md)

[34-Tutorial-HashSet-onwards.md](/documentation/34-Tutorial-HashSet-onwards.md)

## Current program file:

[programiz/attempt-01](/programiz/attempt-01)

____

## notes on running

```bash
cd programmiz/attempt-01
rustc rust.rs
./rust.exe
```

first command goes to folder where `rust.rs` file is

second command compiles using `rustc` command to run rust compiler

third command runs the newly compiled `rust.exe` file

- Note: 

    - It is necessary to add `./` before `rust.exe` to run `rust.exe`

____
