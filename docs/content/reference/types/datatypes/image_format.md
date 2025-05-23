---
title: "ImageFormat"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

The metadata describing the contents of a [`components.ImageBuffer`](https://rerun.io/docs/reference/types/components/image_buffer).

## Fields
#### `width`
Type: `uint32`

The width of the image in pixels.

#### `height`
Type: `uint32`

The height of the image in pixels.

#### `pixel_format`
Type: nullable [`PixelFormat`](../datatypes/pixel_format.md)

Used mainly for chroma downsampled formats and differing number of bits per channel.

If specified, this takes precedence over both [`datatypes.ColorModel`](https://rerun.io/docs/reference/types/datatypes/color_model) and [`datatypes.ChannelDatatype`](https://rerun.io/docs/reference/types/datatypes/channel_datatype) (which are ignored).

#### `color_model`
Type: nullable [`ColorModel`](../datatypes/color_model.md)

L, RGB, RGBA, …

Also requires a [`datatypes.ChannelDatatype`](https://rerun.io/docs/reference/types/datatypes/channel_datatype) to fully specify the pixel format.

#### `channel_datatype`
Type: nullable [`ChannelDatatype`](../datatypes/channel_datatype.md)

The data type of each channel (e.g. the red channel) of the image data (U8, F16, …).

Also requires a [`datatypes.ColorModel`](https://rerun.io/docs/reference/types/datatypes/color_model) to fully specify the pixel format.


## Arrow datatype
```
Struct {
    width: uint32
    height: uint32
    pixel_format: nullable uint8
    color_model: nullable uint8
    channel_datatype: nullable uint8
}
```

## API reference links
 * 🌊 [C++ API docs for `ImageFormat`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1datatypes_1_1ImageFormat.html)
 * 🐍 [Python API docs for `ImageFormat`](https://ref.rerun.io/docs/python/stable/common/datatypes#rerun.datatypes.ImageFormat)
 * 🦀 [Rust API docs for `ImageFormat`](https://docs.rs/rerun/latest/rerun/datatypes/struct.ImageFormat.html)


## Used by

* [`ImageFormat`](../components/image_format.md)
