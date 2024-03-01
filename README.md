## SOFTWARE IS NOT FINISHED!

#### Embrace the power of backticks! Placing \` at both ends of your input line turns the remainder into a code snippet. For instance, typing \`for instance\` results in formatted code. 
#### Additionally, unleash the heading magic by using \# followed by a number to specify the heading level.
##### For example, #4 translates into \#\#\#\#. A single \#, on its own, transforms into a solitary hashtag. Get ready to elevate your markdown writing experience!

### Usage:
```
$ cd marker && rustc -C opt-level=3 src/main.rs -o marker 
$ ./marker
$ #6 Code: hello world 
$ `rs                             # to enter multi line code mode (for rust code)
$ println!("hello, world");
$ `                               # once again to exit the mode
$ q                               # to quit and save this to test.md file
```

### the [Marker](https://github.com/rakivo/marker) will interpret that to this:
###### Code: hello world
```rs
println!("hello, world");
```

### Do this to check:
```shell
$ cat test.md
```

###### ..The essence of this entire project to create a ligthweight-nonoverbloated crafting a tool that enables faster markdown file creation through the use of intuitive hotkeys and other innovative features.
###### By the way this project doesn't use any third party dependencies. It's only std lib of Rust
