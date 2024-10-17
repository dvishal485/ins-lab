# Information And Network Security

Lab file for Information And Network Security (CO425) including programs for various basic ciphers, and algorithms studied during the course.

---

## Programs

All the programs are written in Rust :crab:

To run an example program (say Hill Cipher), locate the corresponding example in [examples directory](examples/), and run the file as follows

```bash
cargo r -r --examples hill_cipher
```

This runs the code for `hill_cipher` in release mode using `cargo`.

### Sample Output

```txt
Finished `release` profile [optimized] target(s) in 0.01s
Running `target/release/examples/hill_cipher`
--------------------------------------------------
Hill Cipher Example
--------------------------------------------------

Key Matrix:
[[13, 8, 13],
 [9, 0, 10],
 [8, 11, 11]]

Inverse Key Matrix:
[[18, 17, 20],
 [5, 13, 13],
 [15, 3, 8]]

Plain text: HELLOCUTEPLANET
Cipher text: GRNVPEWMXXFHGVT
Decrypted text: HELLOCUTEPLANET
```

---

## Lab File

The lab file is generated using [assets/experiments.toml file](assets/experiments.toml) with the help of Typst CLI.

The PDF for lab file can be generated using the following command

```bash
ls examples/ -1 | cut -d "." -f 1 | xargs -I {} sh -c 'cargo r -r --example {} > {}.log 2>&1'
typst c file.typ --ppi 288 "INS Lab File.pdf"
```

This will run all the programs and store its output in a log file and then generate the pdf file `INS Lab File.pdf` as output in the root directory.

---
