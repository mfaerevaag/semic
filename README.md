Semi-C Interpreter
===============

TODO

## Build
Requires rust stable.

Build with (takes a couple of minutes due to parser generator)

    cargo build --release

Binary then found in `target/release/semic-interp`.

To run interactively, use the `--debug` option

    target/release/semic-interp --debug <program.semic>

To test the program given in the assignment, run

    target/release/semic-interp --debug examples/final.semic

See the `examples/` directory for other programs to test.


## Testing
The interpreter is heavily tested through unit and integration tests, found in the `tests` directory.

Run with

    cargo test


## Optional features
This section describes the optional features implemented by this interpreter.

### Global variables
Variables can both be declared and initialized in the global scope.
```c
int foo;

int main(void) {
    foo = 7;

    printf(foo) // 7

    return 0;
}
```

They will automatically be shadowed by variables in a more immediate scope
```c
int foo;

int main(void) {
    foo = 7;

    printf(foo) // 7

    int foo = 42;

    printf(foo) // 42

    return 0;
}
```

This can be tested by running the `examples/global.semic` program

    $ target/release/semic-interp examples/global.semic

### Command line arguments
The `main` function can either be declared as taking the arguments `(int argc, char **argv)`, in the same fashion as C.
```c
int main(int argc, char **argv) {
    int i;

    for (i = 0; i < argc; i++) {
        printf(argv[i]);
    }

    return 0;
}
```

This can be tested by running the `examples/args.semic` with some arguments

    $ target/release/semic-interp examples/args.semic foo bar baz

### Improved scoping
When using commands like `trace` and `print`, the scope is also denoted.

The scopes are either
 - Local
 - Invisible
 - Global

The invisible scope is used when a variable is in the symbol table, but not accessible by the current scope. Here is an example
```c
int a;

void foo(void) {
    int c = 3;

    return;
}

int main(void) {
    a = 1;

    int b = 2;

    foo();

    return 0;
}
```

This can be tested by interactively running the `examples/scopes.semic` program
```sh
$ target/release/semic-interp -d examples/scopes.semic
>> next 7
>> print a
 1 (global)
>> print b
 2 (invisible)
>> print c
 3
```

### Arrays and pointers
Functions can return pointers, as below
```c
char *str(char a, char b) {
    char s[3];
    s[0] = a;
    s[1] = b;
    s[2] = '\0';
    return s;
}

int main(void) {
    printf(str('a', 'b'));
    return 0;
}
```

This can be tested by running the `examples/ptr.semic` program

    $ target/release/semic-interp examples/ptr.semic

### Optional curly braces
As with C, keywords like `if`, `for` and `while`, can take a single argument or a several surrounded by curly braces
```c
int main(void) {
    if (1)
        return 0;
    else {
        return 1;
    }
}
```

This can be tested by running the `examples/blocks.semic` program

    $ target/release/semic-interp examples/blocks.semic

### Error handling
The interpreter handles a variety of errors and prints where the error occurs, when possible.

Below are some examples of how different errors are printed, with the programs found in the `examples` directory (suffixed with `\*_error.semic`)

#### Parse errors
```sh
$ target/release/semic-interp examples/parse_error.semic
Syntax error: line 4:10 (examples/parse_error.semic)
 |     for (i;;) return;
 |           ^
   > Unrecognized token ";". Expected either ["\"++\"", "\"--\"", "\"=\""]
```

#### Checker errors
```sh
Type error: line 5:0 (examples/checker_error.semic)
 | int foo() {
 | ^
   > Function 'foo' already declared
Type error: (examples/checker_error.semic)
   > Function 'main' missing
```

#### Runtime errors
```sh
Run-time error: line 8:8 (examples/runtime_error.semic)
 |     i = foo();
 |         ^
   > Function 'foo' missing param '(int, "a")'
```

### Recursion
The interpreter supports recursion, which can be demonstrated by implemented the Fibonacci function
```c
int fib(int n) {
    if (n <= 1) {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
```

This can be tested by running the `examples/fib.semic` program

    $ target/release/semic-interp examples/fib.semic
