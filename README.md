## SOFTWARE IS NOT FINISED!

### The point of the whole project is to create a tool, that allows you too write md. files faster than usual using hotkeys and things like that.

#### If you put \` into the beginning and the end of yout inpur line, this will interpret the rest of the input as line of code: like that ```for instance```, if you put \# and number of heading level(optionally), for instance: \#4 this will interpret this as \#\#\#\#. (single # without any numbers will be interpreted as single #).

### Usage:
```shell
$ cd marker && cargo run --release
$ #6 Code: hello world 
$ \` # to enter multi line code mode
$ println!("hello, world");
$ \` # once again to exit the mode
$ q # to quit and save this to test.md file
```

### the Marker will interpret that to this markdown code:
```md
###### Code: hello world
```
println!("hello, world");
```
```
