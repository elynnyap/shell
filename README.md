swish
==========

A shell for wizards, witches, and other magical beings.

## Use
```cargo build && /target/debug/main```

To unlock the shell, first tap it with your wand [and recite the magic words](http://harrypotter.wikia.com/wiki/Marauder%27s_Map). To exit, do the same.

## Glossary

In case you failed your OWLs:

Wizard-speak | Muggle-speak
--- | ---
[apparate](http://harrypotter.wikia.com/wiki/Apparition) | cd
[evanesco](http://harrypotter.wikia.com/wiki/Evanesco) | &
[lumos](http://harrypotter.wikia.com/wiki/Wand-Lighting_Charm) | ls
[prior incantato](http://harrypotter.wikia.com/wiki/Reverse_Spell) | history 

## Notes
If run with ```cargo run```, signal handling (ctrl+c) will not work (see [here](https://github.com/Detegr/rust-ctrlc/issues/15) and [here](https://www.reddit.com/r/rust/comments/6lsead/problems_with_ctrlc_handling_under_rust_in_windows/) for explanation). 

## Credits
* [David Evans' Rust Class](http://www.rust-class.org/pages/ps2.html)
