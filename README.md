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

### Update 2024-03-01: 
> - Added ls command to display the whole input you've entered.
> - Added flags, use ```$ ./marker w``` to write or ```$ ./marker a``` to append to the file. you can optionally use another pretty good flag, which can create and write to the file named based on your input, for instance: ```$ ./marker w hello```, this will create file "hello.md" in your current directory.
> - You can move your "cursor" with arrows, but now you have to press enter to got it work (no raw mode).
> - After you moved cursor you can edit the line your cursor in with 'e' command, just type e and your further entry. 
> #### Examples using this: 
```
$ ./marker
$ #6 Code: Testing update 
$ `py
$ print("helo");`        # you can close multi-line code backticks like that
$ ls                     # to print out the whole input we've entered
$ ^[[A # you can simply press arrow up key to write this thingy
$ e print("hello");      # to edit current line
$ q                               
```

###### ..The essence of this entire project to create a ligthweight-nonoverbloated crafting a tool that enables faster markdown file creation through the use of intuitive hotkeys and other innovative features.
###### By the way this project doesn't use any third party dependencies. It's only std lib of Rust
