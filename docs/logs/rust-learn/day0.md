# Day 0: better windows tools

[Back to log main page](../rustlang-learn.md)

## Things I hate on Windows

- no decent shell, powershell is meh, Linux subsystem isn't well integrated
- can't use ls, git, cat, vim, touch, tree, tig, curl and so on with completion alongside Windows commands from cmd.exe
- no package manager to install stuff without downloading and installing like a caveman
- missing a good IDE out of the box to be productive

## How to fix the shell ?

Install the **full version** of [cmder](https://github.com/cmderdev/cmder) to get official msys-git full support and unix shell commands in a single package.

Add it to the Windows context menu, find **cmder.exe** start is as Administrator, then run ```.\cmder.exe /REGISTER ALL ```. 

Then we replace **cmd.exe** to never see it again ! Open Regedit (as Administrator) ```<HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths>```

Add a new Key ```cmd.exe``` (e.g. ```HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\App Paths\cmd.exe```)

Edit the default String value as ```C:\apps\cmder\cmder.exe```, add a new String value named ```Path``` and set it to ```C:\apps\cmder``` so the executable has a PATH including this folder when executed.

### How to fix the package manager ? *(optional)*

You can also install [chocolatey](https://github.com/chocolatey/choco) if you prefer to realy on a package manager instead of downloading and installing apps the old and clunky way. This optional step will actually help you to install rust and other things. But it's fully optional (I didn't do it for this marathon).

### How to fix the IDE ?

Install VScode [https://code.visualstudio.com](https://code.visualstudio.com/), and install all the necessary plugins for this project

- Markdown: [Markdown All in One](https://marketplace.visualstudio.com/items?itemName=yzhang.markdown-all-in-one), [Markdown Preview Enhanced](https://marketplace.visualstudio.com/items?itemName=shd101wyy.markdown-preview-enhanced)
- Rust: [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- TOML: [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- Utilities: [VS Code Counter](https://marketplace.visualstudio.com/items?itemName=uctakeoff.vscode-counter)

### Prepare the coding folder

It's best to create a folder to contain all projects folders and files 

```sh
$ mkdir rust-learn
$ git init
```

Create a ```.gitignore``` file
```sh
$ echo **/target > .gitignore
$ echo **/*.lock >> .gitignore
$ git add *
$ git commit -m "init: initial commit for Rust Learn"
```

You are ready! Just make sure to create all your rust project folder in ```/rust-learn``` and everything will be smooth :)



[Back to log main page](../rustlang-learn.md)