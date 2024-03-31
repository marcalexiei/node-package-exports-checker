# node-package-exports-checker

Utility to check that all field in package.json exports have a corresponding file

## Why?

I want to create something very simple to start learning Rust

## TODO

- [ ] write code for the utility
  - [x] 1. read a package.json
  - [x] 2. check if there is an exports key
  - [ ] 3. Add strong types to json
  - [ ] 4. recursively navigate exports key
  - [ ] 5. when a string is found check that the file specified in it exists
- [ ] add format (check rust fmt package for this)
- [ ] setup this project as a monorepo (see `vercel/turbo` or `biomejs` as example)
- [ ] add unit test
- [ ] add Github Actions
- [ ] add documentation
