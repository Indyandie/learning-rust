# Learning Rust

Let's see how this goes. Start [here](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021). 

## Reference

### Easy Rust

I'm starting with [Dave MacLeod's](https://github.com/Dhghomon) [easy-rust](https://dhghomon.github.io/easy_rust/Chapter_1.html). So far it's been a great starting point, at minimum it removed the deep fear I had about rust. 🦀 Crabs can be pretty scary! 

---

## Notes

### Run a .rs file

```shell
rustc <file-name>.rs && ./<file-name>
```
  
Saving out files to `\bin` dir so I can gitignore them. 

```shell
rustc --out-dir bin <file-name>.rs && bin/<file-name>
```

