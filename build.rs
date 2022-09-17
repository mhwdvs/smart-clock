use std::env;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=rpi-rgb-led-matrix/");

    env::set_var("CXX", "arm-linux-gnueabihf-g++-10");

    env::set_var(
        "CXXFLAGS",
        "-W -Wall -Wextra -Wno-unused-parameter -O3 -g -fPIC -fno-exceptions -std=c++11 -lrt -lm -lpthread",
    );

    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .cpp(true)
        .define("HARDWARE_DESC", "adafruit-hat")
        .include("rpi-rgb-led-matrix/include")
        .file("rpi-rgb-led-matrix/lib/bdf-font.cc")
        .file("rpi-rgb-led-matrix/lib/content-streamer.cc")
        .file("rpi-rgb-led-matrix/lib/framebuffer-internal.h")
        .file("rpi-rgb-led-matrix/lib/framebuffer.cc")
        .file("rpi-rgb-led-matrix/lib/gpio-bits.h")
        .file("rpi-rgb-led-matrix/lib/gpio.cc")
        .file("rpi-rgb-led-matrix/lib/gpio.h")
        .file("rpi-rgb-led-matrix/lib/graphics.cc")
        .file("rpi-rgb-led-matrix/lib/hardware-mapping.c")
        .file("rpi-rgb-led-matrix/lib/hardware-mapping.h")
        .file("rpi-rgb-led-matrix/lib/led-matrix-c.cc")
        .file("rpi-rgb-led-matrix/lib/led-matrix.cc")
        .file("rpi-rgb-led-matrix/lib/multiplex-mappers-internal.h")
        .file("rpi-rgb-led-matrix/lib/multiplex-mappers.cc")
        .file("rpi-rgb-led-matrix/lib/pixel-mapper.cc")
        .file("rpi-rgb-led-matrix/lib/thread.cc")
        .file("rpi-rgb-led-matrix/lib/utf8-internal.h")
        .compile("rgbmatrix");
}
