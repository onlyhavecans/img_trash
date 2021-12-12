# img_trash

Just me playing with destroying and making odd art with images for fun and non-profit.

This code is super ugly since it's just me playing with rust and fighting the compiler.


## jumble
takes a list of files on the command line

Generates pngs of the image with all the pixels randomly jumbled, total static

## line_juggle
takes an optional `skip` option on the command line


Randomly jumbles up all the horizontal lines in each photo in ./img and saves them back out to ./img_out
If skip is the last option passed to the command it only jumbles every other line.