# Rust Learn Book Marathon activity log

> I made and followed [this template](docs/logs/template.md) to document each day of this journey. I plan to follow it on future learning marathons if successful !

## What ?

**This is it !** My second and hopefully successful attempt to learn rust (rustlang) as quickly as possible !

My whole process, and every day of learning will be described in this activity log, so others can benefit too and see if this is a hard path or not.


## Why ?

I have to write fast small programs/scripts for several tasks from work and personal projects alike. These have all become quite pressing due to pandemic lockdown, so I finally mustered the courage to go beyond the cargo Hello world tutorial, and go all the way to that web server project at the end of the "Learn Rust" book !

I dread having to go back to c++ and python or go for my needs but at least I will be sure that they are the right tools with clear first-hand insights.

## How ?

**I am a hardcore procrastinator** on hobby projects and stuff learning endeavors (10+ bought but never started courses on udemy, safari, countless books on many languages both for programming and fellow humans).

I will be pushing everything on my github and logging advances. I expect this strict regimen and rush learning with daily writing will consolidate my grasp of things.

**Environement**

Windows 10 (I am doing this from a clean [Shadow VM](https://www.shadow.tech) to have a destroyable, non-distracting environment to work from and because there is no Linux VM support 'yet' ;-) I will use Windows - *disclaimer: I work at [Shadow](https://twitter.com/ideasmashup)*)

**Constraints**

Finish the whole book in 14 days max, less = better.

<br>

## Devlog

### Day 0 : better Windows 10 tools

> Duration: 30 minutes

Because Windows sucks, and I must code on it... I did a couple tweaks to make thing less awful. I describe them here, and did it all the day before starting my actual rush, to be ready for battle when waking up ! ;)

#### Things I hate on Windows

- no decent shell, powershell is meh, Linux subsystem isn't well integrated
- can't use ls, git, cat, vim, touch, tree, tig, curl and so on with completion alongside Windows commands from cmd.exe
- no package manager to install stuff without downloading and installing like a caveman
- missing a good IDE out of the box to be productive

#### How to fix the shell ?

Install the **full version** of [cmder](https://github.com/cmderdev/cmder) to get official msys-git full support and unix shell commands in a single package.

Add it to the Windows context menu, find **cmder.exe** start is as Administrator, then run ```.\cmder.exe /REGISTER ALL ```. 

Then we replace **cmd.exe** to never see it again ! Open Regedit (as Administrator) ```<HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths>```

Add a new Key ```cmd.exe``` (e.g. ```HKEY_LOCAL_MACHINE\Software\Microsoft\Windows\CurrentVersion\App Paths\cmd.exe```)

Edit the default String value as ```C:\apps\cmder\cmder.exe```, add a new String value named ```Path``` and set it to ```C:\apps\cmder``` so the executable has a PATH including this folder when executed.

#### How to fix the package manager ? *(optional)*

You can also install [chocolatey](https://github.com/chocolatey/choco) if you prefer to realy on a package manager instead of downloading and installing apps the old and clunky way. This optional step will actually help you to install rust and other things. But it's fully optional (I didn't do it for this marathon).

#### How to fix the IDE ?

Install VScode [https://code.visualstudio.com](https://code.visualstudio.com/), and install all the necessary plugins for this project

- Markdown: [Markdown All in One](https://marketplace.visualstudio.com/items?itemName=yzhang.markdown-all-in-one), [Markdown Preview Enhanced](https://marketplace.visualstudio.com/items?itemName=shd101wyy.markdown-preview-enhanced)
- Rust: [Rust (rls)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- TOML: [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml)
- Utilities: [VS Code Counter](https://marketplace.visualstudio.com/items?itemName=uctakeoff.vscode-counter)

<br>

### Day 1 : this day in 5 words

#### Recap

- **Goal**
    - ...
- **Results**
    - ...

> Duration: 0 hours
> Total projects: 0
> Total lines of code: 0

**What I learned**

- [ ] ...
- [ ]

**What I need to improve**

- [ ] ...
- [ ]

**Notes to self**

- ...

**Useful stuff**

- ...

#### Tasks / projects

##### Task / project 1

Quick summary of project or task

> Good :D
> - ...
> - ...

> Bad :/
> - ...
> - ...

```
/// example of representative code ?
```

##### Task / project 2

Quick summary of project or task

> Good :D
> - ...
> - ...

> Bad :/
> - ...
> - ...

```
/// example of representative code ?
```

<br>

(TODO: add extra days here, follow template)

---

(TODO: add aftermath here, follow template)