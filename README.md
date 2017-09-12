swish
==========

A shell for wizards, witches, and other magical beings.

## Use
```cargo build && /target/debug/main```

To unlock the shell, first tap it with your wand [and recite the magic words](http://harrypotter.wikia.com/wiki/Marauder%27s_Map). To exit, do the same.

## Glossary

In case you failed your OWLs:

Spell | Muggle Translation
--- | ---
[accio](http://harrypotter.wikia.com/wiki/Summoning_Charm) | wget
[alohomora](http://harrypotter.wikia.com/wiki/Unlocking_Charm) | open
[aparecium](http://harrypotter.wikia.com/wiki/Revealing_Charm) | cat
[apparate](http://harrypotter.wikia.com/wiki/Apparition) | cd
[avada kedavra](http://harrypotter.wikia.com/wiki/Killing_Curse) | kill
[confundo](http://harrypotter.wikia.com/wiki/Confundus_Charm) | rev
[crucio](http://harrypotter.wikia.com/wiki/Cruciatus_Curse) | yes
[depulso](http://harrypotter.wikia.com/wiki/Banishing_Charm) | ping
[evanesco](http://harrypotter.wikia.com/wiki/Evanesco) | &
[gemino](http://harrypotter.wikia.com/wiki/Gemino_Curse) | cp
[imperio](http://harrypotter.wikia.com/wiki/Imperius_Curse) | sudo
[legilimens](http://harrypotter.wikia.com/wiki/Legilimency) | less
[locomotor](http://harrypotter.wikia.com/wiki/Locomotion_Charm) | mv
[lumos](http://harrypotter.wikia.com/wiki/Wand-Lighting_Charm) | ls
[obliviate](http://harrypotter.wikia.com/wiki/Memory_Charm) | clear
[portus](http://harrypotter.wikia.com/wiki/Portus) | ln
[prior incantato](http://harrypotter.wikia.com/wiki/Reverse_Spell) | history 
[reducio](http://harrypotter.wikia.com/wiki/Shrinking_Charm) | tar
[reducto](http://harrypotter.wikia.com/wiki/Reducto) | rm
[reparo](http://harrypotter.wikia.com/wiki/Mending_Charm) | fsck
[scourgify](http://harrypotter.wikia.com/wiki/Scouring_Charm) | rmdir
[sectumsempra](http://harrypotter.wikia.com/wiki/Sectumsempra) | cut
[stupefy](http://harrypotter.wikia.com/wiki/Stunning_Spell) | sleep

## Notes
If run with ```cargo run```, signal handling (ctrl+c) will not work (see [here](https://github.com/Detegr/rust-ctrlc/issues/15) and [here](https://www.reddit.com/r/rust/comments/6lsead/problems_with_ctrlc_handling_under_rust_in_windows/) for explanation). 

## Credits
* [David Evans' Rust Class](http://www.rust-class.org/pages/ps2.html)
* http://www.chris.com/ascii
* J.K. Rowling
