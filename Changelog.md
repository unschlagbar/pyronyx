# Changelog

## [0.2.4]

### Added

Default & null() to PhysicalDevice
Implemented `From<Extent2D> for Rect2D`

### Changes

All method params that where `[c_void]` are now `[u8]`

## [0.2.3]

### Added

vk::Result type as wrapper for Result<T, vk::Error>
vk::Error now implements std::error::Error;


## [0.2.2]

### Added

Missing len functions.

### Fix

Builing on systems where c_char = u8, Raspberry pi in my case

## [0.2.1]

### Added Error Messages for not loaded functions

Not loaded core functions throw `CORE_LOAD_ERROR` on use.
Not loaded extension functions throw `EXT_LOAD_ERROR` on use.

### Fix

Some inconstistent function docs
