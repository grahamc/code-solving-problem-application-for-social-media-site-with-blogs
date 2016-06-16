# generator.py

Generates `example-logfile-0000` through `example-logfile-0009`.
Spreads the log entries around from morning to night (across one day,)
with randomly selected error codes across the day.

# parse.py

Finds a list of files (globbed on: `./example-logfile-*`) and extracts
the date (up to/including the minute) and status code.

With 10 files, it takes about 45 seconds to produce `./out.csv`.

## Parsing
There are two implementations of the parser, and the regex
implementation is the in-use version. The other two are kept to
demonstrate experiments in parsing, to try and get better parsing
performance. All three perform about equally on my machine, which
surprised me.

### regex
Two are in the code, one is a bit more complex (but perhaps less
naive.) This took about 6-7 seconds to process 1,000,000 rows so I
replaced it with the more naive one below it. The more naive one
parses 1,000,000 rows in about 2-3 seconds.

### scan
Worried invoking the regex regex engine was too much, I tried just
scanning the strings with index().

### split
I wasn't sure how the implementation of split fares against indexing,
but again the perfomrance was about on par with the regex option.

## Multiprocessing
I gave it a go with multiprocessing and forking of workers to parse
individual jobs. At 10 ~500mb files, it took a very long time to
serialize and return the data from the workers.

For the code sample, I decided to leave out multiprocessing and leave
it as a conversation.
