## Moonlight

This project is my second attempt at writing a ray tracer. This time,
I wanted to experiment with adding an Entity-Component-System design
pattern with [specs](https://slide-rs.github.io/specs/).

ECS is a pattern used in video games to separate behaviour from state
and favour composition over inheritance. For example, there's an
entity for the camera that is composed of some components, like:
`Transform`, `Velocity`, `FieldOfView`, `Image`, etc.

A sphere or a light could also have a `Transform` or a `Velocity`
which determine where they're placed in the scene and the direction
they're moving. This means a `Physics` system can update the
`Transform` for all components with a `Velocity`, without caring what
those entities are.

This lets you break things down into 'systems' that do small chunks of
data processing, while not having global visibility over everything,
meaning you can reason about what each thing does more easily. In this
project:

- The `SceneGenerator` is a standalone system that creates spheres /
lights / cameras

- The `RayTracer` loops through all cameras, traces rays through their
pixels and writes the result into the Image component

- The `Physics` system just moves the camera

- The `ImageWriter` loops through all images and writes them to png
files

At present, if you run this code, it writes some `.png` files to the
current director. These can be stitched into a video with ffmpeg:

```
$ cargo run --release

$ ffmpeg -y -framerate 60 -pattern_type glob -i '*.png' -c:v libx264 -crf 18 -pix_fmt yuv420p video.mp4
```

[Here's an example video.](video.mp4)

Currently, the ray tracer in this project is quite minimal. It doesn't
support reflections, etc. This project is mostly a proof-of-concept.
