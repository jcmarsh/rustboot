# rustboot

Look into https://github.com/jvns/puddle

A tiny 32 bit kernel written in Rust.

I was inspired to download Rust and try to do this after seeing [zero.rs](https://github.com/pcwalton/zero.rs) - a stub that lets Rust programs run almost freestanding.

It paints the screen bright red and then hangs. That's it:

![](http://i.imgur.com/NWRehJJ.png)

## Interesting forks

* [jvns/puddle](https://github.com/jvns/puddle)

* [pczarn/rustboot](https://github.com/pczarn/rustboot)

## Setup

You need a few things to run rustboot:

1. `qemu`
2. a cross-compiler for i386
3. `nasm`
4. Rust's `master` branch or 0.7 release.

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install qemu
```

Make sure the brew version of `nasm` is being used:

```bash
$ nasm -v
NASM version 2.11.02 compiled on Apr 14 2014
```

Install binutils from source.

I personally keep things I manually compile limited to my home directory, so
I use the `--prefix=/Users/steve` option. Put this wherever you want, of
course.

```bash
$ wget http://ftp.gnu.org/gnu/binutils/binutils-2.24.tar.gz
$ tar xf binutils-2.24.tar.gz
$ cd binutils-2.24
$ ./configure --target=i386-elf --disable-werror --prefix=/Users/steve
$ make && make install
```

To get edge Rust going, grab it from git:

```bash
$ git clone https://github.com/mozilla/rust
$ cd rust
$ ./configure --prefix=/Users/steve
$ make && make install
```

Same thing about the prefix applies.

Then, just make sure that `~/bin` is in your `PATH`, if you're using a prefix.

## Running it

To compile, simply

```bash
$ make
```

To run,

```bash
$ make run
```
