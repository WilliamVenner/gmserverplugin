# gmserverplugin

This is a utility library for making [Server Plugins](https://developer.valvesoftware.com/wiki/Server_plugins) that access the Lua state in Garry's Mod.

Currently, accessing the Lua state from a server plugin before it has been created yet requires hooking various internal functions to "steal" the Lua state pointer once it's been initialized. The problem with this is that if two server plugins do this, they'll end up changing the assembly of the function and the signature to find that function will no longer work.

There's a [simple solution](https://github.com/Facepunch/garrysmod-requests/issues/1917), but we can safely assume that the request will never be fulfilled. If it is, this repo will be archived for obvious reasons.

In the mean time, I have created this statically linked hacklioteque that will handle all the bullshit for you. You can use this library in any language you want, as long as it supports FFI with C.

# Usage

If you're not using Rust, you can download a precompiled static library from the [releases page](https://github.com/WilliamVenner/gmserverplugin/releases).

## Events

There are three events you can hook into.

| Event | Description |
|:---:|:---:|
| `newstate` | Called when the Lua state is created and we have a pointer to it.<br>At this point, the Lua state is completely naked - no libraries have been loaded, no Lua scripts have been executed. |
| `before_init` | Called before includes/init.lua is executed.<br>At this point, stuff like `SERVER` and `NULL` have been defined in the global state. |
| `after_init` | Called after includes/init.lua is executed.<br>At this point, we've loaded all the Lua extension in Garry's Mod into the Lua state. The gamemode will be loaded shortly after this event. |

## Rust

#### Cargo.toml

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
gmserverplugin = "*"
# OR
gmod = { version = "*", features = ["server-plugin"] } # The gmod crate will re-export gmserverplugin for you with the `server-plugin` feature. https://github.com/WilliamVenner/gmod-rs
```

#### lib.rs

```rust
extern "C" fn newstate(lua_state: *mut std::ffi::c_void) {
    println!("newstate");
}

extern "C" fn before_init(lua_state: *mut std::ffi::c_void) {
    println!("before_init");
}

extern "C" fn after_init(lua_state: *mut std::ffi::c_void) {
    println!("after_init");
}

#[no_mangle]
unsafe extern "C" fn CreateInterface() -> *mut std::ffi::c_void {
    gmserverplugin::init(); // This will hook the relevant functions, if not already

    gmserverplugin::newstate(newstate); // Register a callback that will be called when the `newstate` event fires.
    gmserverplugin::before_init(before_init); // Register a callback that will be called when the `before_init` event fires.
    gmserverplugin::after_init(after_init); // Register a callback that will be called when the `after_init` event fires.

    // create an IServerPluginCallbacks instance and return a pointer to it
    // you'll probably want to use cxx for this!
}
```

## C++

```cpp
extern "C" {
    extern "C" typedef void (*Callback)(void* lua_State);

    void init();
    void newstate(Callback);
    void after_init(Callback);
    void before_init(Callback);
}
```

# Important Notes

Be careful with managing your state between map changes. You should test this thoroughly. Callbacks will still fire between map changes. You cannot unregister callbacks.
