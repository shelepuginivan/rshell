# rshell documentation

## 1. Base system commands

To use commands such as `mkdir`, `mv` and other executables, just write it with arguments:

```shell
mkdir test
echo Hello, World!
```

You also can run other executable files:

```shell
vim test.txt
```

## 2. Shell built-in commands

`rshell` provides some built-in commands:

* `cd` - change directory
* `set` - set a variable (see variables)
* `alias` - set an alias (see aliases)
* `fn` - declare a function (see functions)
* `exit` - terminate this process and exit the shell

## 3. `&&` operator

`&&` (or logical AND) is used when we want to run commands until one of them ends with non-zero exit code

```shell
echo both && echo printed
```

*stdout:*

```
both
printed
```

And an example where one command finishes with error

```shell
not_existing_command && echo won't be printed
```

*stderr:*

```
rsh: No such file or directory (os error 2)
```

## 4. Sequential execution (`;`)

Sequential execution is useful if we execute more than one command regardless of how the previous command ended

```shell
echo printed; echo each; echo again
```

*stdout:*

```
printed
each
again
```

This will happen if any command finishes with error

```shell
echo anyway; cd not_existing directory; echo will output this
```

*stdout:*
```
anyway
will output this
```

*stderr:*
```
rsh: No such file or directory (os error 2)
```

And this is what you will see in terminal:

```
anyway
rsh: No such file or directory (os error 2)
will output
```

## 5. Comments

`rshell` supports comments. To write a comment, use `#`:

```
# this will be ignored

echo some text # as well as this
```

*stdout:*

```
some text
```



## 6. Pipes

Pipes allow you to redirect `stdout` of any process to `stdin` of another one. 

Here is an example of using pipes:

```shell
echo message | bat
```

```
───────┬────────────────────────────────────────────────
       │ STDIN
───────┼────────────────────────────────────────────────
   1   │ message
───────┴────────────────────────────────────────────────
```

You can use pipe operator more than one tome if you need. Note that only `stdout` is redirected, `stderr` is not.

## 7. I/O redirection

I/O redirection is used for redirecting output of the process, usually to the file

Example:

```shell
echo new > new.txt
cat new
```

*stdout:*

```
new
```

`>` operator overwrites the file while `>>` appends stdout to the file

```
echo newline >> new.txt
cat new.txt
```

*stdout:*

```
new
newline
```


It also can be used multiple times

```
echo another >> new.txt > really_new.txt
cat new.txt; echo; cat really_new.txt
```

*stdout:*

```
new
newline
another

new
newline
another
```

Yes, unlike many other shells, it actually writes (or appends) the whole file content to the next file.

## 8. Script execution

`rshell` can execute scripts written in its language.

Create a file with `*.rsh` extension and paste there this code:

```shell
echo Hello, World! | @format --red --bold
```

Than, execute this file with:

```
rsh your_file_name.rsh
```

*stdout:*

```
Hello, World!
```

* Note that the output should be bold and red-colored 

So, `rshell` tries to act as an interpreted programming language

## 9. Profile

Profile is a file that is executed as the shell process started.

You can modify it as you wish, for exapmle, set aliases, variables or functions, or do anything you need.

Profile is accesseble with `$profile` env variable.

You can also find it in `~/.rsh_profile`.

## 10. Variables

You can set the variable with `set` keyword:

```rshell
set <key>=<value>
```

Now you can get your variable

```rshell
echo $<key>
```

*stdout:*

```
<value>
```

You also can set the variable value as another variable:

```rshell
set some_var=variable
set another_one=$some_var
echo $another_one
```

Will print:

```
variable
```

Or you can set a key as a variable value

```
set $another_one=works

echo $variable
```

*stdout:*

```
works
```

## 11. String literals

If you want to use the *exact* string, not a variable, you can use string literals.

Put `'` before the string to make it literal

```rshell
set v=some_var

set v1='$v

echo $v1
```

*stdout:*

```
$v
```

## 12. Aliases

If you want to use one command/function instead of another one, you can set an alias

```rshell
alias cat=bat
```

Now, `bat` will be executed instead of `cat`

If you want to use cat, use string literals:

```rshell
'cat some_file.txt
```

Aliases also supports args, for example:

```rshell
alias ll=exa -la
```

Now, `ll` command will execute `exa` with `-la` flags

## 13. Functions

`rshell` supports functions. To declare a function, use
`fn` keyword, name and either block of commads or inline command:

```rshell
fn func {
echo a
}
```

or

```
fn func echo a
```

In first case, shell will write your input as the function body until you close the bracket. The prompt will look like `>` symbol.

```rshell
func
```

*stdout:*

```
a
```

Functions also supports argument. To declare arguments in the fucntion body, use `&` char:

```rshell
fn newEcho {
echo &arg
}

newEcho text
```

*stdout:*

```
text
```

You can use as many arguments as you want, but if you call a function with fewer arguments, the remainder will be interpreted as the `&<argname>` string.


## 14. Instants

Instants are a sort of built-in functions. They can take arguments from pipe but they also print the result directly to *stdout* and it cannot be piped.

Instants' names starts with `@`.

In the current `rshell` version, there are only 2 instants:

* `@exec` - executes `*.rsh` file in the current process. You also can use variables, aliases and functions that vere declered in this file, so it is very similar to Python `import` operator (since it executes imported file).

`@exec` instant has one parameter `--noexit`. If provided, shell won't finish its process even if file includes `exit` command.

Usage:

```rshell
@exec <filename> [--noexit]
```

* `@format` - formats the string. We used it in script execution section. It supports many arguments and actually can change string color and other parameters like font width.

Here is the table with `@format` arguments:


| argument | effect |
|:----------------|:--------|
| --red | change color to red |
| --green | change color to green |
| --yellow | change color to yellow |
| --blue | change color to blue |
| --magenta | change color to magenta |
| --cyan | change color to cyan |
| --white | change color to white |
| --bold | **increases** font width |
| --italic | change font style to *italic*
| --underline | change font style to <ins>underline</ins> |
| --dimmed | decreses text brightness |

Usage:

```rshell
@format <any text> [--arg1 [--arg2 [...]]]
```
