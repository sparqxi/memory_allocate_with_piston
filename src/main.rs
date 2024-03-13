use graphics::{clear, rectangle};
use piston_window::{PistonWindow, WindowSettings};
use world::World;
use std::{
    alloc::{GlobalAlloc, System},
    time::Instant,
};
mod world;
mod particle;

#[global_allocator]
static ALLOCATOR: ReportingAllocator = ReportingAllocator;
struct ReportingAllocator;

unsafe impl GlobalAlloc for ReportingAllocator {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let start = Instant::now();
        let ptr = System.alloc(layout);
        let end = Instant::now();
        let time_taken = end - start;
        let bytes_requested = layout.size();
        eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: std::alloc::Layout) {
        System.dealloc(ptr, layout);
    }
}

fn main() {
    let (width, height) = (1280.0, 960.0);
    let mut window:PistonWindow = WindowSettings::new("particles", [width, height])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window.");

    let mut world = World::new(width, height);
    world.add_shapes(1000);
    while let Some(event) = window.next() {
        world.update();
        window.draw_2d(&event, |ctx,renderer,_device| {
            clear([0.15,0.17,0.17,0.9], renderer);
            for s in &mut world.particles {
                let size = [s.position[0],s.position[1],s.width,s.height];
                rectangle(s.color, size, ctx.transform, renderer);
            }
        });
    }
}
