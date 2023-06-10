![tender-mule](https://github.com/stvsmth/tendermule-rs/assets/3330805/2e8a01b3-098f-44d9-a925-cc851fe0fe0f)

# tendermule

`tendermule` is a simple utility that generates unique identifiers with some constraints. Why the quirky name? 
Well, I generated a couple dozen names using this tool and `TenderMule` spoke to me.

This tool exists because I was automating a task that needed a unique identifier less than 16
characters. Over the course of my work, I would have more than a dozen such identifiers, and I
wanted something more memorable than `MyTask-1234`. I also wanted a customizable prefix, so I could
group my tasks together, as well as alert others on the team that these were my tasks.

This started as a Python project, but I moved it to Rust so that I could distribute it as a
self-contained binary (or even an embedded library).

Currently, each identifier will always be in the form of `AdjectiveNoun`, and will always have the
first letter of each word capitalized (camel case). You can supply a prefix or suffix of up to five
characters; we honor the case of the prefix and suffix inputs.

You may request more than one identifier. The results are guaranteed to be unique;
however, your combination of maximum length, prefix, and suffix could result in limited
(or even no) results.

## Usage

Generate one unique identifier:
```
$ tendermule
WastefulGuppy
```

Generate 4 unique identifiers, with a prefix
```
$ tendermule --prefix stv --count 4
stvCurvyDancer
stvSteadyDiamond
stvSecretLoon
stvHuskyCentaur
stvElderlyBoar
```

See `--help` for other options.

## Can I ...

* Pass in my own list of adjectives and nouns
* Filter out certain adjectives or nouns from the provided list
* Fill the result to some fixed length (with random characters)
* Control the case of the identifier

The answer to all of the above is the same: Not yet, but probably later.
