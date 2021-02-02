# png2linetext

Transform PNG to textual line art!

Images require pre-processing to come out nice. It's expected that users will do
the following:

1. Apply gaussian blur to remove noise and details.
2. Remove the color from the image.
3. Resize the image to 240px width (80 characters) and round the height to the
nearest number divisible by 3.
4. Apply an edge detection algorithm.
5. Invert the image to see the line art.
6. Run this program on the image.

Usage is simple: `png2linetext image.png`

If the width and height are not divisible by 3, the program will crash.

## Explanation

The program simply runs over the whole image, trying to match a "mask" and then
prints the corresponding matching character. Because the characters are chosen
to make lines stand out, the result should look better for line-oriented output.

This page was used to gather symbols to use for drawing:
https://en.wikipedia.org/wiki/Box-drawing_character .

## Examples

Becase web browsers are currently terrible at rendering the output as all
monospace, unlike terminals, the examples below have been included as images. If
you work for Google or Mozilla or Microsoft or Apple please fix this for the
love of god.

![](./inputs/bird1bit.png)
![](./examples/1612243569.png)
![](./inputs/bloodhound1bit.png)
![](./examples/1612075258.png)

## Against the competition

The source image:

![](./inputs/lobsters1bit.png)

The 1st is latest png2linetext algorithm vs whatever I could quickly find:

![](./examples/1612243605.png)

vs https://manytools.org/hacker-tools/convert-images-to-ascii-art/go/

![](./examples/1612074741.png)

vs https://www.text-image.com/convert/pic2ascii.cgi

![](./examples/1612075210.png)

vs https://www.ascii-art-generator.org/

![](./examples/1612075258.png)

vs https://www.topster.net/ascii-generator/

![](./examples/1612075432.png)

vs http://www.glassgiant.com/ascii/ascii.php

![](./examples/1612075461.png)


