Spellcheck
==========

[![Build Status](https://travis-ci.org/past/spellcheck.svg?branch=master)](https://travis-ci.org/past/spellcheck)
[![Latest version](https://img.shields.io/crates/v/spellcheck.svg)](https://crates.io/crates/spellcheck)
[![Documentation](https://docs.rs/spellcheck/badge.svg)](https://docs.rs/spellcheck)
![License](https://img.shields.io/crates/l/spellcheck.svg)

This is a Rust implementation of Peter Norvig's statistical spell-checking
algorithm. You can read more about the approach in the original article:

[http://norvig.com/spell-correct.html](http://norvig.com/spell-correct.html)

This is a port of the JavaScript implementation of the algorithm from
[https://github.com/past/speller](https://github.com/past/speller)

Contents
--------

* src/lib.rs is the spell-checker implementation 
* src/bin/speller.rs is a command-line program that can be used as a demo of the library
* src/bin/training.txt is Peter Norvig's original training file, with about a million words (obtained by converting the original file in the JavaScript implementation via `iconv -c -f ASCII -t UTF-8 ../speller/bin/big.txt > training.txt`

License
-------

Copyright (c) 2018 Panagiotis Astithas

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
