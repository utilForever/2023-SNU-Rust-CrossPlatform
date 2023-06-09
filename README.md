# 2023-SNU-Rust-CorssPlatform

2023년 서울대학교 컴퓨터공학부 특강 - Rust 크로스 플랫폼 프로그래밍 발표 자료 및 예제 코드

## Contents

- [Presentation](./1%20-%20Presentation/SNUCSE%20Speical%20Lecture%20-%20Rust%20Cross%20Platform%20Programming.pdf)
- [Example](./2%20-%20Example/)

## Prerequisite

1. Install [Rust](https://www.rust-lang.org/learn/get-started).

### Rust + iOS

1. Install [Xcode](https://developer.apple.com/xcode/).

2. Install Xcode Command Line Tools:

```sh
xcode-select --install
```

3. Add iOS architectures to support cross-compile:

```sh
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

4. Install [cargo-lipo](https://github.com/TimNN/cargo-lipo) for making universal library easily:

```sh
cargo install cargo-lipo
```

### Rust + WebAssembly

1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/) for making WebAssembly binary file easily.

2. Install [node](https://nodejs.org/en/) and [npm](https://www.npmjs.com/) using homebrew:

```sh
brew install node
```

3. Install [yarn](https://yarnpkg.com/) using homebrew:

```sh
brew install yarn
```

## Quick Start

### Rust + iOS

1. Run this command inside the `rust-cross-ios` directory:

```sh
cargo-lipo --release
```

2. That's it! You can run iOS example in Xcode.

### Rust + WebAssembly

1. Run this command inside the `rust-cross-web` directory:

```sh
wasm-pack build
```

2. Copy `pkg` folder to `example-web` directory.

### WebAssembly with TypeScript

1. Ensure that the local development server and its dependencies are installed by running this command within the `example-web` directory:
```sh
npm install
```

2. Run this command from within the `example-web` directory:
```sh
npm run start
```

3. Go to `http://localhost:8080/`

### Generating Foreign-Language Bindings with UniFFI

#### Kotlin

```sh
cargo run --bin uniffi-bindgen generate src/calc.udl --language kotlin
```

#### Swift

```sh
cargo run --bin uniffi-bindgen generate src/calc.udl --language swift
```

## References

- Rust Core
  - Beginner
    * [The Rust Programming Language](https://doc.rust-lang.org/book/)
    * [Rustlings](https://github.com/rust-lang/rustlings/)
    * [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
    * [Rust-101 by Ralf Jung](https://www.ralfj.de/projects/rust-101/main.html)
    * [Exercism - Rust](https://exercism.org/tracks/rust)
    * [Book: The Rust Programming Language](http://www.yes24.com/Product/Goods/83075894)
    * [Book: Rust in Action](https://www.manning.com/books/rust-in-action)
    * [Book: Programming Rust](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/)
  - Intermediate
    * [The Standard Library](https://doc.rust-lang.org/std/index.html)
    * [The Edition Guide](https://doc.rust-lang.org/edition-guide/index.html)
    * [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
    * [The rustdoc Book](https://doc.rust-lang.org/rustdoc/index.html)
    * [The rustc Book](https://doc.rust-lang.org/rustc/index.html)
    * [Book: Concurrent Programming](http://www.yes24.com/Product/Goods/108570426)
    * [Book: Rust for Rustaceans](https://rust-for-rustaceans.com/)
  - Advanced
    * [The Rust Reference](https://doc.rust-lang.org/reference/index.html)
    * [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html)
    * [The Rust Unstable Book](https://doc.rust-lang.org/nightly/unstable-book/index.html)
- Rust + iOS
  * [Building and Deploying a Rust library on iOS](https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html)
- Rust + WebAssembly
  * [Rust and WebAssembly](https://rustwasm.github.io/docs/book/)
  * [Book: The Art of WebAssembly](https://nostarch.com/art-webassembly)
  * [Book: WebAssembly: The Definitive Guide](https://www.oreilly.com/library/view/webassembly-the-definitive/9781492089834/)

## How To Contribute

Contributions are always welcome, either reporting issues/bugs or forking the repository and then issuing pull requests when you have completed some additional coding that you feel will be beneficial to the main project. If you are interested in contributing in a more dedicated capacity, then please contact me.

## Contact

You can contact me via e-mail (utilForever at gmail.com). I am always happy to answer questions or help with any issues you might have, and please be sure to share any additional work or your creations with me, I love seeing what other people are making.

## License

<img align="right" src="http://opensource.org/trademarks/opensource/OSI-Approved-License-100x137.png">

The class is licensed under the [MIT License](http://opensource.org/licenses/MIT):

Copyright &copy; 2023 [Chris Ohk](http://www.github.com/utilForever).

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
