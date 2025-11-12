
# 🐚 0-shell — A Minimalist Unix-like Shell in Rust

### Authors
👨‍💻 **Omar Ait Benhammou**
👨‍💻 **Oussama Benali**
👨‍💻 **Mohamed El-Fihry**
👨‍💻 **Ibrahim El Harraq**

---

# 🚧 Project Status: Work in Progress  

We’re still actively **developing and improving 0-shell**.  
Our team is continuously refining command handling, optimizing performance, and adding new features such as **better error handling, I/O redirection, and enhanced piping support**.  

Stay tuned — future updates will bring more stability, usability, and functionality!


## 🧭 Overview

**0-shell** is a minimalist Unix-like shell implemented entirely in **Rust**, designed to run on Unix systems without depending on existing shells (like `bash` or `sh`) or external binaries.

Our shell handles **core file system operations**, **process control**, and **command execution** using Rust’s **system-level abstractions**, ensuring **safety**, **robustness**, and **efficiency** — ideal for embedded Linux or lightweight system environments.

---

## ⚙️ Features

### ✅ Core Commands Implemented

Each command is written from scratch using Rust’s standard library and low-level system calls:

| Command | Description                           | Supported Options |
| ------- | ------------------------------------- | ----------------- |
| `echo`  | Prints text to standard output        | —                 |
| `cd`    | Changes the current working directory | —                 |
| `pwd`   | Prints the current working directory  | —                 |
| `ls`    | Lists directory contents              | `-l`, `-a`, `-F`  |
| `cat`   | Displays file contents                | —                 |
| `cp`    | Copies files                          | —                 |
| `mv`    | Moves or renames files                | —                 |
| `rm`    | Removes files or directories          | `-r`              |
| `mkdir` | Creates directories                   | —                 |
| `exit`  | Exits the shell                       | —                 |

---

## 🧩 Bonus Feature: Piping (`|`)

We implemented **command piping**, allowing output from one command to serve as input to another.

### Example:

```bash
$ echo "hello world" | cat
hello world
```

Our shell uses **inter-process communication (IPC)** via **Unix pipes** and **Rust’s `std::os::unix::io` API** to connect child processes efficiently.

---

## 🧠 Learning Objectives

Through this project, we learned how to:

* Use **system calls** for file and process management in Rust
* Manage **user input** and shell loops
* Handle **process creation** and synchronization (`fork`, `exec`, etc.)
* Implement **error handling** and graceful exit on EOF (`Ctrl+D`)
* Explore **Unix design principles** and shell behavior

---

## 💻 Usage

### Build

```bash
make
```

### Run

```bash
make run
```

### Example Session

```bash
$ pwd
/home/student
$ mkdir test && cd test
$ echo "Rust is fast!" > file.txt
$ cat file.txt
Rust is fast!
$ ls -l
-rw-r--r-- 1 user user 13 Nov 12 15:30 file.txt
$ echo "piping works" | cat
piping works
$ exit
```

---

## ⚠️ Error Handling

Unrecognized commands are handled gracefully:

```bash
$ something
Command 'something' not found
```

Pressing `Ctrl+D` exits the shell without crashing.

---

## 🎨 Future Improvements

Potential enhancements include:

* **Auto-completion** for commands and paths
* **Command history**
* **Environment variables** (`$HOME`, `$PATH`)
* **Colorized prompt and output**
* **I/O redirection (`>`, `<`)**
* **Command chaining (`;`)**

---

## 🧰 Technical Details

* **Language:** Rust
* **System API:** POSIX (via `std::os::unix` and `nix` crate equivalents if used)
* **Memory Safety:** Fully guaranteed by Rust ownership model
* **Platform:** Linux / Unix-based systems

---

## 📦 Repository

🔗 [0-shell on Zone01 Oujda Git](https://learn.zone01oujda.ma/git/oaitbenh/0-shell)

---

## 🧾 License

This project is developed for educational purposes as part of the **Zone01 Oujda** curriculum.
All contributors hold equal ownership of the code.

