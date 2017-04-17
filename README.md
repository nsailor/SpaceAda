# SpaceAda

SpaceAda is an attempt to create a very simple language resembling Ada, targeting embedded architectures.
It's specification should be very simple so that programs written in it can be easily verified and statically analyzed.

The main idea is that SpaceAda programs can be analyzed using SPARK on a desktop platform, and then compiled by `spadac` for any LLVM architecture, including AVR microcontrollers.

This will enable the Arduino to run SPARK-proven code.

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

