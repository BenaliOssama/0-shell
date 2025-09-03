// <from> https://micahkepe.com/blog/minishell/?utm_source=chatgpt.com#executing-commands

Executing Commands
We now have a basic shell that can read input from the user and parse that input into a sequence of commands that can be executed by spawning new processes. However, not all commands are equally handled by the shell, leading to the need for built-in commands and understanding how shells create processes.

How Shells Create Process
Before we execute commands, I think a little background on how shells create processes is in order. When a shell executes a command, it typically does so by creating a new process, a process being an instance of a running program.

In Unix-like systems, this is done using the fork and exec system calls:

Fork: The shell creates a new process by duplicating itself using the fork system call. This creates a child process that is an exact copy of the parent shell process (this will become important later).
Exec: The shell then replaces the child process's memory space with the new command using the exec system call. This means that the child process is now running the new command, but it still has the same process ID (PID) as the original shell.
As a result, the child process can run independently of the parent shell, and the shell can continue to run and accept new commands. When the child process finishes executing the command, it can return an exit status to the parent shell, which can then display the result to the user.

Even though these details are abstracted away in Rust, they are still important to understand how our shell will work. When we execute a command, we will use the Command struct from the std::process module, which internally handles the fork and exec system calls for us. The Command struct provides a convenient way to spawn new processes and pass arguments to them.


