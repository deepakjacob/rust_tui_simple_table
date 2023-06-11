# Table Viewer Application

This is a simple Rust application that displays a table using the Cursive library. The table view dislplays following on a terminal for a table with columns for Name, Age, and City. It uses ncurses by default, but [other backends are available](https://github.com/gyscos/cursive/wiki/Backends).


| Name    | Age | City      |
| ------- | --- | --------- |
| Alice   | 25  | New York  |
| Bob     | 30  | London    |
| Charlie | 35  | Tokyo     |


## Dependencies

To build and run this application, you need to have Rust installed on your system. You can install Rust by following the instructions at [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

This application also depends on the Cursive library for creating the text-based user interface. The Cursive library is included as a dependency in the `Cargo.toml` file.

To add the Cursive dependency to your Rust project, open the `Cargo.toml` file and add the following line to the `[dependencies]` section:

```toml
cursive = "0.20"
```


To execute the application, 
```code 
cargo run
```


To exit the application,
```code
press the q key
```


