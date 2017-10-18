# Nucleo Rust Project 

A simple application for the Nucleo F302-R8 using Xargo, svd2rust, and
the [cortex-m-quickstart](https://github.com/japaric/cortex-m-quickstart).

There's this [pretty neat tutorial by Jorge Aparicio](http://blog.japaric.io/quickstart/) that covers setting it up for an F3 discovery board.

Right now it just blinks an LED with a very manual delay (after some
hardware modifications).

Also requires running svd2rust on the STM32F302x SVD. And unfortunately
it appears that doens't have `enumeratedValue` tags.
