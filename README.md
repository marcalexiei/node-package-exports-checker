# node-package-exports-checker

Utility to check that all field in package.json exports have a corresponding file

## Why?

I want to create something very simple to start learning Rust

## Todo

- [ ] setup this project as a monorepo (see `vercel/turbo` or `biomejs` as example)
- [ ] write code for the utility
  1. read a package.json and check if there is a export key
  2. recursively navigate exports key
  3. when a string is found check that the file specified in it exists
- [ ] add format (check rust packages for this)
- [ ] add unit test
- [ ] add Github Actions
- [ ] add documentation (with deploy on github pages)
