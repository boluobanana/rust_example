fn main() {
  cc::Build::new()
      .cpp(true) // Switch to C++ library compilation.
      .file("src/test.cpp")
      .compile("libtest.a");
}