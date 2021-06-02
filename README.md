# sent_parse

This crate takes a string (e.g. from a text file) and parses it into a vector of presentation slides in the style of [sent](https://tools.suckless.org/sent/). With this, you can implement display features without worrying about parsing the text.

Each slide is returned as a struct with associated metadata. Currently there is a ``TextSlide`` struct and an ``ImageSlide`` struct.

For example, your program can read ``presentation.txt`` into a string and then provide it to ``sent_parse``. But this library isn't opinionated about the source of its input, so you can use it ways that diverge from the original suckless tool.
