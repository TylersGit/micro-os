# What is this?

A minimal OS built in Rust.

## Why is this?

There is a well loved rust OS Dev blog, which has rightfully earned its place in the discussion.  
It is very well written, and I can easily understand the choices that were made there.

However, I quickly ran into issues as I am not a fan of running the nightly versions of software.   
I was not able to get the tooling required for the project to run without switching to nightly, so I figured I'd make it a point of prinicple to write an OS without any nightly features in the tech stack. 

## Required software
* [Just](https://github.com/casey/just) - The command runner.  
* grub - Specifically for the grub-mkrescue util.
* xorriso - A dependency of grub-mkrescue. 
* qemu - For virtualization.
* grub-pc-bin - grub will not load properly otherwise. 