![tender-mule](https://github.com/stvsmth/tendermule-rs/assets/3330805/2e8a01b3-098f-44d9-a925-cc851fe0fe0f)

# tendermule

This is a simple utility that generates unique identifiers with some constraints. Why is
it called `tendermule`. I generated a couple dozen names using this tool and that one
spoke to me.

I wrote this because I was automating some tasks that needed to be unique and needed to be under
automating certain length. Because I would have 6-15 of these items defined, I wanted something
more memorable than MyTask-1234. I also wanted a prefix, so I could group my tasks together as well
as alert others on the team that these were my tasks.

But mostly I wanted to tinker in Rust.

The identifier will always be in the form of `AdjectiveNoun`. Currently, it will always
be in snake case. You can supply a prefix or suffix of up to five characters; we honor
the case of the prefix and suffix inputs.

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
* Filter out cert adjectives or nouns
* Fill the result to some fixed length (with random numbers)
* Control the case of the identifier

The answer to all of the above is the same: Not yet, but it's coming.
