# Project 1 Chicken and Rabbit Problem
## Introduction
The Chicken and Rabbit Problem is a classic math problem:  
A farm has some rabbits and chickens. Together they had `x` heads and `y` legs. How many chickens and rabbits are in the farm?

The program allows user to provide the number of "heads" and "legs", then calculates the number of chickens and rabbits.
In the complete version of the program, we will deploy it on the cloud server and allow user to use it through the URL address.

![picture1](https://user-images.githubusercontent.com/84234596/214658730-dbfa1288-881b-493a-b61c-314d9985ddc5.png)

## Usage
At this time, this is a command line program. Users can run it in the command line and provide the number of heads and legs, then the program will return the number of each aniaml.
In complete version, users could play with it through link address.
1. Run it on Terminal
- Type command: `cargo build --release`
2. Run it in Docker
- Type command: `docker build -f Dockerfile . -t rust-autocomplete:latest`
## Examples
1. Input the number of heads and legs
```Rust
Please enter the number of heads:
5
Please enter the number of legs:
16
```
2. Get the result
```Rust
There are 3 rabbits and 2 chickens in the farm.
```
## To do
- To deploy on AWS Cloud Server
- To allow user to provide the input in the URL as the format `URL/heads-legs`.(heads for head number and legs for leg number)

## Reference
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [rust-new-project-template](https://github.com/noahgift/rust-new-project-template)
* [chicken-and-rabbit-cage-problem](https://github.com/KaijianHuang/chicken-and-rabbit-cage-problem)
