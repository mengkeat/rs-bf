# rs-bf
Brain***k in Rust: Getting to know Rust with a toy implementation of BF.

### TO Run 
Command line: rs-bf \<bf-file\>

### Note:
- BF memory location is at least according to Wikipedia (https://en.wikipedia.org/wiki/Brainfuck) of
  byte sized of length 30,000. This implementation allows explicitly for the overflow and underflow. Some BF code seems
  to depend on this (Mandelbrot for example).

### Current ideas for further exploration:
- There are clearly some obvious optimizations that can be done i.e https://www.nayuki.io/page/optimizing-brainfuck-compiler
- Setting up proper testing structures
- JIT ? 
