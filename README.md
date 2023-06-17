<p align="center"> 
	<img src="asset/logo.png" width=160 height=160  >
</p>
<div align="center">
    <img alt="License" src="https://img.shields.io/github/license/mumu12641/strawberry?color=red&style=flat-square">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/mumu12641/strawberry?color=red&style=flat-square">
<h1 align="center">
	Strawberry
</h1>
<p align="center">
  A toy object-oriented programming language written by rust
</p>
</div>

## 🍓Why strawberry

Just because I happened to be eating strawberries when I had this idea!

## 🌟 About the Project

This is a toy object-oriented language, a practice project after I finished learning CS143, there are many features it does not support, and API will keep changing until version1.0 (if I can do this), so please just think of it as a small language for learning.

## 🎯Features
- :white_check_mark: Object-oriented and Polymorphism(I guess).

- :white_check_mark: Output readable x86 assembly code and run it natively.

- :white_check_mark: Type inference at variable definition time.

- :construction: Support for generics.

- :construction: More APIs.


## ☀ Getting Started

### ❗️ Prerequisites

Your computer should be a linux system, forgive me for not being able to support windows. This project requires that your PC be installed with gcc and cargo.Use following commands to check it!

```
gcc -version
cargo --version
```

If you successfully display the version information, then move on.

### ⚙️ Installation

1. Clone the repo

   ```
   git clone https://github.com/mumu12641/strawberry.git
   ```

2. Install

   ```
   make install
   ```

### 👀Usage

1. Create project

   ```
   strawberry new example
   ```

2. Build it

   ```
   cd ./example
   strawberry build
   ```

3. Run it

   ```
   ./build/a.out
   ```

   
