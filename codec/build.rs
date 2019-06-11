fn main() {}
/*
extern crate cc;


fn test() {
    cc::Build::new()
        .file("libs/ffmpeg/libavformat/format.c")
        //.file("libs/ffmpeg/libavutil/log.c")
        .include("libs/ffmpeg")
        .compile("format.a");
}

#[allow(dead_code)]
fn test() {
    cc::Build::new().file("libs/doubler.c").*//*file("libs/main.c").*//*compile("foo.a");
}

#[allow(dead_code)]
fn gl() {
    extern crate gl_generator;

    use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
    use std::env;
    use std::fs::File;
    use std::path::Path;

    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gl, (4, 5), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}*/
