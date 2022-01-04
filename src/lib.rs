#![feature(c_unwind)]

#[macro_use] extern crate gmod;

#[lua_function]
unsafe fn hello_world(lua: gmod::lua::State) -> i32 {
    // lua_run hello_world()

    lua.get_global(lua_string!("print"));
    lua.push_string("Hello, world!");
    lua.call(1, 0);
    return 0;
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
    println!("My {} module version {} has loaded!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    lua.push_function(hello_world);
    lua.set_global(lua_string!("hello_world"));

    return 0;
}

#[gmod13_close]
unsafe fn gmod13_close(lua: gmod::lua::State) -> i32 {
    println!("My {} module version {} has unloaded!", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    return 0;
}