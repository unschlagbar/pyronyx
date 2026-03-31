# Pyronyx

Lightweight fully generated next gen Vulkan bindings for Rust — focused on zero overhead and natural Rust ergonomics.

[![Vulkan 1.4](https://img.shields.io/badge/Vulkan-1.4-darkred.svg)](https://registry.khronos.org/vulkan/)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE-MIT)

## Overview

- [x] Generated from `vk.xml` & `video.xml` — complete Vulkan 1.0 through 1.4 coverage
- [x] Full Vulkan Video support
- [x] Commands dispatch from the handle they belong to
- [x] Extensions are just methods — enable them in `CreateInfo`, call them directly
- [x] Zero-cost handles — a vtable pointer and nothing else
- [x] Slices instead of count + pointer pairs
- [x] `Result`-returning commands
- [x] No hidden runtime cost

<p align="center">
  <img src="https://www.nvidia.com/content/dam/en-zz/Solutions/geforce/news/vulkan-graphics-api-launches-nvidia-gpus-game-ready/vulkan-logo-featuredmain.png" height="200" />
  <img src="https://d29g4g2dyqv443.cloudfront.net/sites/default/files/akamai/VulkanVideo_Logo-512.png" height="200" />
</p>

---

## Everything is on the right handle

`Device`, `PhysicalDevice`, `Instance`, `Queue`, `CommandBuffer` — every handle knows its commands directly. There is nothing to construct beyond the handle itself:

```rust
// physical device queries
let caps = physical_device.get_surface_capabilities(surface)?;
let formats = physical_device.get_surface_formats(surface)?;
let props = physical_device.get_properties();

// device commands
let swapchain = device.create_swapchain(&create_info, None)?;
let images = device.get_swapchain_images(swapchain)?;

// queue
queue.submit(&[submit_info], fence)?;

// command buffer
command_buffer.bind_pipeline(PipelineBindPoint::Graphics, pipeline);
command_buffer.bind_vertex_buffers(0, &[vertex_buffer], &[0]);
command_buffer.draw_indexed(index_count, 1, 0, 0, 0);
```


## Extensions — no loaders required

Enable an extension in `CreateInfo` and its functions are immediately available on the handle. No separate objects, no extra allocation:

```rust
let extensions = [surface::NAME.as_ptr(), wayland_surface::NAME.as_ptr()];

let create_info = vk::InstanceCreateInfo {
    enabled_extension_names: extensions.as_ptr(),
    enabled_extension_count: extensions.len() as u32,
    ..Default::default()
};
let instance = vk::Instance::create(&create_info, None)?;

// surface and debug functions are already on instance
let surface = instance.create_wayland_surface(&surface_info, None)?;
instance.destroy_surface(surface, None);
```

Same for device extensions — `khr::swapchain`, `khr::acceleration_structure`, anything:

```rust
let device = physical_device.create_device(&create_info, None, &instance)?;

// no SwapchainDevice::new(), just call it
let swapchain = device.create_swapchain(&swapchain_info, None)?;
device.destroy_swapchain(swapchain, None);
```

## Zero-cost handles

Every handle is a raw Vulkan handle plus a single pointer to its vtable — nothing else. No `Arc`, no reference counting, no allocation. Calling a method is one pointer dereference, identical to calling the function pointer directly.


## Slices and Vec returns

```rust
let images = device.get_swapchain_images(swapchain)?;  // Vec<Image>
let devices = instance.enumerate_physical_devices()?;  // Vec<PhysicalDevice>

// count/pointer pairs become slices
command_buffer.bind_descriptor_sets(
    PipelineBindPoint::Graphics,
    layout,
    0,
    &descriptor_sets,
    &[],
);
```


## Vulkan Video

Complete generated bindings for video:

```rust
use pyronyx::vk::video::*;

let profile = VideoProfileInfoKHR {
    video_codec_operation: VideoCodecOperationFlagsKHR::DecodeH264,
    ..Default::default()
};
```


## Pointer chains

Use `base.next(ext)` to insert ext into the start of the pointer chain attached to base.

```rust
let mut features11 = vk::PhysicalDeviceVulkan11Features {
    shader_draw_parameters: vk::TRUE,
    ..Default::default()
};

let create_info = vk::DeviceCreateInfo {
    enabled_extension_names: extensions.as_ptr().cast(),
    enabled_extension_count: extensions.len() as u32,
    // ...
    ..Default::default()
}.next(&mut features11);

```

---

## Why not Ash?

Ash has been effectively unmaintained a good time now. Beyond that, its model requires a separate loader struct per extension — `khr::Swapchain::new(&instance, &device)`, `ext::DebugUtils::new(&entry, &instance)` — which you then carry alongside your device for the lifetime of your program. Command buffer commands live on `Device` rather than `CommandBuffer` because that is where the loader is.
`Device` is just the huge struct that you need everywhere

Pyronyx does not have that distinction. Enable an extension, call its functions on the handle, done.

---

## Example 

### [Triangle](https://github.com/unschlagbar/pyronyx-triangle)

Hello Triangle: [pyronyx Triangle](https://github.com/unschlagbar/pyronyx-triangle)

![Hello Triangle](https://raw.githubusercontent.com/unschlagbar/pyronyx-triangle/main/image.png)

## License

MIT
