# Vigenère cipher (simple solver)
This is an attempt on creating a simple [Vigenère cipher][wikipedia] solver.

While being at a conference, I was searching for a fun programming challenge.
A friend of mine provided a challenge from his university (TU Delft), which was
about finding an encryption key for a Vigenère ciphertext.
I included the given ciphertext [here](./ciphertext.txt).

For this attempt I assumed the decrypted text and the encryption key were
British English words, and assumed these words were included in the dictionary
that was available on my system. I've included this dictionary in the repository
[here](./dictionary.txt). Therefore I choose to do a dictionary attack for the
key, and to compare part of the decrypted ciphertext for each attempt to the
same dictionary to test whether the attempted key may be valid.

Yes, this is a brute force attack. Implementing some form of statistical
analysis on this cipher text would probably perform much better. This is however
a fairly simple implementation which is awesome in it's own way. Also, if the
assumption of the key and the decrypted text being British English was
incorrect, the program wouldn't be able to find the correct key. The
implementation does successfully find the key however, therefore I didn't
implement a more complex solution.

For a benchmark, see the [benchmark](#benchmark) section below.

## Benchmark
Here is a benchmark for this implementation on the challenge
[ciphertext](./ciphertext.txt) with the provided [dictionary](./dictionary.txt)
on a `Intel Core i5-4670K CPU @ 3.40GHz` CPU with 16GB RAM.
These benchmarks were done with the [hyperfine][hyperfine] tool,
each timing 10 runs.

Note that the program attempts all possible words, and doesn't quit after
a possible key has been found (which would make the total process even quicker).

```
hyperfine './target/release/vigenere-cipher-simple'
# Time (mean ± σ):   41.0 ms ±  1.1 ms
# Range (min … max): 40.0 ms … 47.1 ms
# [User: 40.0 ms, System: 47.1 ms]
```

The following answer is outputted:

```
==================
FOUND POSSIBLE KEY!
KEY: BEAUTIFUL [1, 4, 0, 20, 19, 8, 5, 20, 11]
TEXT: AS SUMMER CAME TO AN END, ALL OF THE FLOWERS IN THE GARDEN
WANTED TO KNOW WHICH FLOWER WAS THE BEST: THE PINK ROSES
SAID, ”WE ARE THE BEST BECAUSE WE WERE THE FIRST FLOWERS
TO BLOOM IN THE SPRING.” THE WHITE DAISIES SAID, ”OH NO, WE
ARE THE BEST BECA... (truncated)
==================
```

For brute forcing I believe this is quite quick, even though it's using a
dictionary attack.

I've attempted a brute force attack on a 32-core Xeon server without
a dictionary and an unspecified key length.
I estimate that this attack would complete in about 1 day for this ciphertext,
although I only let it run for half an hour.

## Performance
To keep the implementation of this solver as performant as possible, I've done
the following things:

- **Rust:** I've used the Rust programming language as it's amazingly quick for
  the high-level abstractions and features it provides.
  This implementation proves that once again :smiley:.
- **Dictionary:** a dictionary attack is used to minimize the number of brute
  force attempts.
- **Concurrency:** all brute force attempts are spread over all available CPU
  cores.
- **Lazy:** while brute forcing lazy iterators are used to only shift
  characters being looked at while searching for a possible solution, instead
  of shifting the whole ciphertext each attempt.
- **Limited success checks:** for each decryption attempt only the first 8 words
  are checked against a provided dictionary, to check for a possible successful
  key. Because of this only a fairly limited selection of the ciphertext is
  decrypted. The dictionary is checked using binary search.
- **Minimal copying:** the number of memory allocations due to copying is
  minimized during decryption attempts.
- **LTO:** link time optimizations are enabled during compilation of release
  builds.
- **Vectorization:** code is optimized to allow compiler vectorization as much
  as possible to fully utilize a CPU core.

## Usage
To run this solver, first make sure [Rust][rust] (`nightly`, `v1.28` or higher)
is installed on your system which may be done using [rustup][rustup].

Run the following commands (Linux/macOS) to compile and run the solver:

```bash
# Clone the repository
git clone https://gitlab.com/timvisee/vigenere-cipher-simple-git
cd vigenere-cipher-simple

# Compile and run
cargo run --release

# or run it directly
cargo build --release
./target/release/vigenere-cipher-simple
```

The challenge ciphertext and the used dictionary are included in this
repository, and the paths to them are hardcoded. To use a different ciphertext
or dictionary, make sure to replace the existing files or to change the paths in
the source code.

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information. 

The included [ciphertext](./ciphertext.txt) was provided by the TU Delft.  
The included [dictionary](./dictionary.txt) originates from [SCOWL][scowl].


[rust]: https://rust-lang.org/
[rustup]: https://rustup.rs/
[hyperfine]: https://github.com/sharkdp/hyperfine
[scowl]: http://wordlist.aspell.net/
[wikipedia]: https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher
