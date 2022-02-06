![Riverbend High School](https://www.spotsylvania.k12.va.us/cms/lib/VA01918722/Centricity/Template/GlobalAssets/images///logos/RHS.png)

# **RHS Library Who Goes There?** Rust Rocket REST API

![Rust 2021](https://img.shields.io/badge/Rust-2021-%232D44A4?style=flat)
![GPLv3 License](https://img.shields.io/badge/License-GPLv3-%232D44A4?style=flat)
![Version 0.1.0](https://img.shields.io/badge/Version-v0.1.0-%232D44A4?style=flat)

This repository contains the backend server code for the student monitoring and session tracking system used in the Riverbend Library. This api is used to implement a simple signin/out system to make sure students are accounted for at all times, including in an emergency.

This backend is running on Rust using [Rocket v0.5.0-rc1](https://rocket.rs/). It is designed to be used with a the frontend component found in [the frontend component folder](/frontend). WGT uses a Vue based implementation. It also uses [PostgreSQL](https://www.postgresql.org/).

Created by [Gabriel Hogan](https://gabrielhogan.com) and [Samuel Rembisz](https://stappsworld.com).


## Installation
All commands assume that you are at the root of the `wgt-backend` repository that you cloned.
1. Ensure you have both [Cargo and Rustup](https://rustup.rs/) to date as well as [PostgreSQL](https://www.postgresql.org/).
    ```
    rustup update
    ```

2. Create and populate the `.env` files.
    ```
    cp .env.example .env
    cp ./frontend/wgt-frontend/.env.example ./frontend/wgt-frontend/.env
    ```

3. Install the Diesel CLI for the database
    ```
    cargo install diesel_cli --no-default-features --features postgres
    ```

4. Make default user in [migrations/.../up.sql](migrations/2022-01-31-022712_students/up.sql)

5. Create database tables.
    ```
    diesel migration run
    ```

6. From there, the program can be run
    ```
    cargo r --release
    ```

## Authentication
Authentication with the API is currently done through **API Tokens** supplied via a query paramater.
(More Info Soon)

## License
Distributed under the GPLv3 License. See [LICENSE](LICENSE) for more information.

## Contact
RHS Library - rhs@spotsylvania@k12.va.us
