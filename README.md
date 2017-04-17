# SpaceAda

SpaceAda is an attempt to create a very simple language resembling Ada, targeting embedded architectures.
It's specification should be very simple so that programs written in it can be easily verified and statically analyzed.

The main idea is that SpaceAda programs can be analyzed using SPARK on a desktop platform, and then compiled by `spadac` for any LLVM architecture, including AVR microcontrollers.

This will enable Arduino to run SPARK-proven code.

## Core Features
#### Comments
Comments begin with a `--` and span the entire line.

### Syntax
SpaceAda supports simple procedures and functions. A procedure is defined using the following syntax:

```
procedure Print_Int(X : in Integer);

procedure Print_Sum(X : in Integer; Y : in Integer) is
	Sum : Integer;
begin
	Sum := X + Y;
	Print_Int(Sum);
end Print_Sum;
```

All statements end with a semicolon.
Functions can be defined in a similar manner:

```
function Square(X : in Integer) return Integer is
begin
	return X * X;
end Square;

function Average(A : in Float; B : in Float) return Float is
begin
	return A * B * 0.5;
end Average;
```

The first version of SpaceAda will support only the `Integer` and `Float` data types.

## Usage

In order to use the runtime functions declared in `code.spad`, you need to link the object file
produced by the compiler with `spada-rt.c`. A typical compilation sequence might look like this:

```
spadac sample.spad -o sample.o
clang -c spada-rt.c -o spada-rt.o
clang sample.o spada-rt.o -o sample
./sample
```

### Installation
To build `spadac` you will need a recent version of Rust.
Due to the quick nature of Rust's development, we recommend
always using the latest version of the language.
The only dependency you need to install manually is the LLVM
development libraries.

On macOS, you can use Homebrew with `brew install llvm`.
On Ubuntu, `sudo apt-get install llvm-dev` should be enough.

Once you clone the repository, you can use `cargo` to build `spadac`:

```
cargo build
cargo install
```

This shouldn't take longer than a couple of minutes.
