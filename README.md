# rust-fibonacci-sequence
Prints a simple fibonacci sequence up to a certain number you input. A lot of fibonacci examples seem to use match, I decided to use a tuple implementation instead since Fibonacci numbers sort of act like they are in pair (n - 2) and (n - 1) get you to the next n since generating a new fibonacci number requires the "pair" of them. This program was written with the intention of learning Rust. It was left as an exercise in the Rust book. (https://doc.rust-lang.org/book/ch03-05-control-flow.html)

# How to run
1. Install Rust (rustup is the easiest way). https://www.rust-lang.org/tools/install
2. Go into the folder and run:

```sh 
cargo run main
```

You should see the following prompt:

```sh
Enter length of Fibonacci sequence to generate: 
```

Enter a number and it will print out the Fibonacci numbers asked for.

```sh
Enter length of Fibonacci sequence to generate: 10
Printing the first 10 numbers of the Fibonacci sequence.
0
1
1
2
3
5
8
13
21
34
```

You can type `exit` or `quit` or press Ctrl-C to exit the program. 
