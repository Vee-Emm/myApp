# myApp
Demo of Smtree from novi

Test 
----
#cargo build
#Execute binary in target/debug/myApp


Output will look like this : 


Hello, world!
[src/main.rs:74] to_hex_string(&tree.get_root_raw().serialize()) = "0ACC9779AB6EBD977EF3B27A5E0A66E9A335F7BC7B341DD093F8E21F4F0FAC7F"
Tree height: 4
               ^
       /               \
       ^               ^
   /       \       /       \
   ^       ^       ^       o
 /   \   /   \   /   \   /   \
 ^   ^   ^   ^   ^   o   .   .
/ \ / \ / \ / \ / \ / \ / \ / \
* * * * * * * * * * . . . . . .
[src/main.rs:29] x = true
[src/main.rs:36] y = false
[src/main.rs:43] a = true
[src/main.rs:50] b = false
