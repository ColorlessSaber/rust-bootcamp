<a id="readme-top"></a>


<!-- Project Shields -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![GPL License][license-shield]][license-url]

<!-- Project Title -->
<br>
<div>
    <h1 style="text-align:center">Let's Get Rust Bootcamp Repo</h1>
</div>

<!-- Table of Contents -->
<details>
    <summary>Table of Contents</summary>
    <ol>
        <li><a href="#about-the-repo">About the Repo</a></li>
        <li><a href="#built-with">Built With</a></li>
        <li><a href="#projects-under-repo">Projects Under Repo</a></li>
        <ul>
            <li><a href="#beginner">Beginner</a></li>
            <li><a href="#intermediate">Intermediate</a></li>
            <li><a href="#projects">Projects</a></li>
        </ul>
        <li><a href="#license">License</a></li>
        <li><a href="#contact">Contact</a></li>
        <li><a href="#acknowledgements">Acknowledgements</a></li>
    </ol>
</details>
<br>

<!-- About the Repo -->
## About the Repo
This repository contains what I have been learning from the [Let's Get Rusty Bootcamp][rust-bootcamp-url],
along with projects created in the bootcamp. Please see the section <a href="#Projects Under Repo">Projects Under Repo</a>
for more details of what I've learned thus far and projects I have accomplished.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Built With -->
## Built With
* [![Rust][rust-shield]][rust-url]
* [![Postgresql][postgresql-shield]][postgresql-url]
* [![Docker][docker-shield]][docker-url]
* [![Axum][axum-shield]][axum-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Projects under repo -->
## Projects Under Repo
### Beginner
Learned the basics of Rust, project structure, and creating unit tests. This section of the bootcamp is broken down into the following sections:
* 01 - Basics: data types, functions, variables, control_flow, constraints and static variables.
* 02 - Memory Safety: Ownership, borrowing, and slices.
* 03 - Custom Data Types: Enums, structs, tuple structs, vectors, match block, and option and result enums.
* 04 - Structure Projects: Learned how to properly structure your project using Cargo.toml and lib.rs.
* 05 - Structure Large Projects: Learned about workspaces, importing creates from Crates.io, and cargo features.
* 06 - Unit Test and Documentation: was taught how to create unit tests and creating documentation.

### Intermediate
In this section of the bootcamp, learned more intermediate topics of Rust.
* 01 - Polymorphism: generics and traits, trait objects, super traits, and deriving traits.
* 02 - Advanced Memory Management: Smart pointers (Box, Rc, RefCell), ellision rules, and concrete and generic lifetimes.
* 03 - Error Handling: Was taught the difference between unrecoverable, recoverable, and propagating errors.
* 04 - Advanced Error Handling: Learned how to create cleaner error messages for user and developer using Rust's built-in traits and create from Crate.io (anyhow, thiserror, and error-stack).
* 05 - Functional Features: Closures, function pointers, iterator patterns, iterating over collections and combinators.

### Advanced
The advanced Rust topics touched by in the bootcamp are listed below:
* 01 - Concurrency: Creating threads, passing state amd messages between threads, learned about Tokio Crate, and Streams.
* 02 - Rust Macros: Declarative Macros and Procedural Macros.
* 03 - Unsafe Rust and FFI: Defer raw pointers, unsafe functions, unsafe traits, and Foreign Function Interface (FFI)

### Projects
#### CLI - Jara Clone
Created a clone of the Jara software--an industry-standard tool for tracking progress of software projects. The
terminal style software allows the user to create Epics and Stories. <br>
See the README file in the project folder for more details.

#### API - Stack Overflow Clone
A clone of the Stack Overflow API. This project allows a person to create new questions and post answers to the
questions. There is also the option to remove a question and or answer. This project was creating using the Axum crate
and utilized the Postgres SQL server.<br>
See the README file in the project folder for more details.

#### Microservice Project
A simple microservice project that implements an authentication, health-status, and a client CLI. Able to run in the
terminal or via Docker.<br>
See the README file in the project folder for more details.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- License -->
## License
Distributed under the MIT License. See 'LICENSE.txt' for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Contact -->
## Contact
Please head to my portfolio website and use the contact form to reach out to me:
[My Portfolio Website][portfolio-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements

* [Choose an Open Source License](https://choosealicense.com)
* [Img Shields](https://shields.io)
* [Let's Get Rusty Bootcamp][rust-bootcamp-url]

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- Markdown Links & Images -->
[contributors-shield]: https://img.shields.io/github/contributors/ColorlessSaber/rust-bootcamp.svg?style=for-the-badge
[contributors-url]: https://github.com/ColorlessSaber/rust-bootcamp/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ColorlessSaber/rust-bootcamp.svg?style=for-the-badge
[forks-url]: https://github.com/ColorlessSaber/rust-bootcamp/network/members
[stars-shield]: https://img.shields.io/github/stars/ColorlessSaber/rust-bootcamp.svg?style=for-the-badge
[stars-url]: https://github.com/ColorlessSaber/rust-bootcamp/stargazers
[issues-shield]: https://img.shields.io/github/issues/ColorlessSaber/rust-bootcamp.svg?style=for-the-badge
[issues-url]: https://github.com/ColorlessSaber/rust-bootcamp/issues
[license-shield]: https://img.shields.io/github/license/ColorlessSaber/rust-bootcamp.svg?style=for-the-badge
[license-url]: https://github.com/ColorlessSaber/rust-bootcamp/blob/main/LICENSE

[rust-shield]: https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=white
[rust-url]: https://www.rust-lang.org
[postgresql-shield]: https://img.shields.io/badge/PostgreSQL-4169E1?style=for-the-badge&logo=PostgreSQL&logoColor=white
[postgresql-url]: https://www.postgresql.org
[docker-shield]: https://img.shields.io/badge/Docker-2496ED?style=for-the-badge&logo=docker&logoColor=white
[docker-url]: https://www.docker.com
[axum-shield]: https://img.shields.io/badge/axum-000000?style=for-the-badge&logoColor=white
[axum-url]: https://crates.io/crates/axum

[portfolio-url]: https://colorlesssaber.github.io/
[rust-bootcamp-url]: https://product.letsgetrusty.com/bootcamp-sp?r_done=1