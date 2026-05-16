# Rust basics 
* [Binding and mutability](#Binding-and-Mutability)
* [Scope](#Scope)
## Binding and Mutability

A variable can be used only if it has been initialised.

![An image showing the error of a used variable that was not initiated](../../images/uninitiated.png)


A variable that has not been initialised but used will yeild a warning on compilation

![An image showing a warning for an uninitiated variable that was not used](../../images/uninitiated_not_used.png)


WHEN THE VARIABLES ARE INITIALISED
![An image of the output when the variable has been initialsed](../../images/a.png) 

## Use mut to mark a variable as mutable

In rust a variable by its nature is immutable and you have to explicity state that you want a variable to be mutable

![An image showing mut being used](../../images/b.png)

## Scope.

Scope of a variable is the block of code in which it is declared.
A variable can  be accessed in its scope.

![An image that illustrates the scope idea in code](../../images/d.png)

![An image of an out of scope variable error](../../images/c.png)

What it looks like when the y variable is removed on the outer scope

![the outer scope without y](../../images/e.png)
