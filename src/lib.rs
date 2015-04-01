#![feature(core)]
#![feature(custom_attribute)]
#![feature(old_path)]
#![feature(plugin)]
#![plugin(gfx_macros)]

extern crate collada;
extern crate gfx_debug_draw;
extern crate vecmath;

// TODO - 'SkinnedRenderer' probably belongs in its own crate,
// then we wouldn't need the following dependencies here

extern crate gfx;
extern crate quack;
extern crate wavefront_obj;
extern crate geometry;
extern crate gfx_device_gl;
extern crate gfx_texture;
extern crate quaternion;


pub mod animation;
pub mod skinned_renderer;
mod math;

pub use animation::{ AnimationClip, AnimationSample };
pub use skinned_renderer::SkinnedRenderer;
