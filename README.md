# `iw` simple image viewer

The idea of the project is to create a simple image viwer on [Rust](https://www.rust-lang.org/) with [GTK3](https://gtk-rs.org/) bindings.

The project is inspired by [Viewnior](https://github.com/hellosiyan/Viewnior) and the aim is to implement just some of the functionality available in the Viewnior.

## The Goal of the Project
The foal of the project is to learn Rust and some GTK3 bindings.
There are no purpose to create fully featured application at the moment.

## Current implementation state

- [x] open image in command line
- [x] `<Left>` and `<Right>` arrow keys to navigate to the previous and next image in a folder
- [ ] implement scroll event for image scaling
	- [x] `<ScrollUP>` and `<ScrollDown>` mouse events to increase and decrease image scale
	- [ ] "smart" scaling (center of scaling is under the mouse pointer)
- [x] positioning of an image at the center of the window while
	- [x] scroll
	- [x] `<Left>`/`<Right>` key press
	- [x] window resize
- [ ] implement image drag
    - [x] implement basic drag
	- [x] drag upscaled image
	- [x] keep image borders inside window while dragging
	- [ ] keep position of dragged image while scrolling
- [ ] set max width and height of the window for images with big resolution
- [ ] set application icon

