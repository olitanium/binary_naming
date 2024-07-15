Program to render numbers as text according to a long-binary naming scheme. A new word is introduced every 2^N trailing zeros.

This system shares similarities with the old English number system, say:

1,101,000,000: (one thousand one hundred and one)-million

In this system, numbers are names thus:

                                                                                1 : one   =>     1
                                                                               10 : two   =>     2
                                                                              100 : four  =>     4
                                                                           1 0000 : hex   =>    16
                                                                      1 0000 0000 : byte  =>   256
                                                            1 0000 0000 0000 0000 : short => 65536
                                        1 0000 0000 0000 0000 0000 0000 0000 0000 : int   => ~4.3e9
1 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 0000 : long  => ~1.8e19

so a number such as 1 0111 1101 0000 0111 (97543) would be:
	short ((four two one)-hex (two one)-four one)-byte four two one.

Whist number systems like this are longer and more cumbersome, there are eight words needed to describe numbers up to 2^128 - 1: 340,282,366,920,938,463,463,374,607,431,768,211,455 as
	seven hundred and fifty eight million, two hundred and eleven thousand, four hundred and fifty five	

versus:
	((((((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-short (((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-int ((((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-short (((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-long
   (((((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-short (((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-int ((((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one)-short (((two one)-four two one)-hex (two one)-four two one)-byte ((two one)-four two one)-hex (two one)-four two one.

This system has less ability to make obvious at the beginning of the number the order of magnitude, as the entire first half (up to the word "long" in the previous example) all give information as to the general size, before the latter half deals with the smaller detail. In life, however, numbers very large only appear in Science, who would still use a scientific exponential notation (but based on powers of two), such that a large number around the length of 'long' would be pronounced as "exp four hex" and a number such as 1.011e100010101101 would be "one point quart (half)-quart exp ((two)-four)-byte ((two)-four two)-hex (two one)-four one". as the "one" prefix is actually implied by the binary scientific notation (as per IEEE-754), the one can be dropped for "point quart (half)-quart exp ((two)-four)-byte ((two)-four two)-hex (two one)-four one"
