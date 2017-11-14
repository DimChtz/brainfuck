# cpp-brainwords (brainwords) - v0.1.0
A simple and easy to use Brainfuck code generator in C++.

# How to use (Hello world! Example)
```cpp
std::cout << generateBFCode("Hello world!") << std::endl;
```

#### Result on the console.
```bash
++++++++[>++++++++<-]>++++++++.<+++++[>+++++<-]>++++.<++[>++<-]>+++.<[><-]>.<+[>+<-]>++.<++++++++[>--------<-]>---------------.<
+++++++++[>+++++++++<-]>++++++.<++[>--<-]>----.<+[>+<-]>++.<++[>--<-]>--.<++[>--<-]>----.<++++++++[>--------<-]>---.<
```


# Installation

Simply include the single header file to your project:

```cpp
#include "brainwords.h"
```
