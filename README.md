# above_me
## Goal
This projects listens to the APRS-servers of the [Open Glider Network](http://wiki.glidernet.org/) and keeps track of all incoming aircraft status. It provides an API that returns the latest status for each aircraft within a given location and range. It also provides a simple website that fetches and lists those aircraft status based on your device location automatically.  

## Set up
### Native
#### Build
##### Backend
- `cd backend/`
- `cargo build --release`

##### Frontend
- `cd frontend/`
- `npm run copy-dependencies`
- `npm run compile`

#### Run
- Either run `RUST_LOG=error cargo run` inside the [backend/](backend) directory or build the backend and run `RUST_LOG=error ./backend/target/release/above_me`.
- Build the frontend and serve the _frontend/dist_ directory.
  (For **development**, you may run `npm run dev` inside the [frontend/](frontend) directory. Requires _Python 3_.)

#### Configuration
Only the backend must be configured. Frontend will run as-is. There are three ways for configuration:

1. _/config.json_ (copy [config.example.json](config.example.json))
2. _/backend/config.json_ (copy [config.example.json](config.example.json), overrides _1._)
3. by environment variables with the prefix _ABOVE\_ME\_\__ (see [/docker/.env.example](docker/.env.example), overrides _1._ and _2._)

### Docker
Configure by setting up _/docker/.env_ (copy [/docker/.env.example](docker/.env.example)) and run `docker compose up`.

(Please note that the _bind\_url_ should be left unconfigured as it is used in the [/docker/docker-compose.yml](docker/docker-compose.yml) config file. Otherwise the proxy-pass may break.)

### Privacy policy
The website contains links to _privacy-policy.html_. You can (and should) set up this privacy policy page. Empty dummy files already exist in the [_docker/_](docker) and [_frontend/src/_](frontend/src) directories.

## API
API-Documentation: [openapi.yml](openapi.yml)

## Status
[![Cargo test & clippy](https://github.com/neon-JS/above_me/actions/workflows/cargo.yml/badge.svg)](https://github.com/neon-JS/above_me/actions/workflows/cargo.yml)  
[![Docker backend - build & push](https://github.com/neon-JS/above_me/actions/workflows/docker-backend.yml/badge.svg)](https://github.com/neon-JS/above_me/actions/workflows/docker-backend.yml)  
[![Docker frontend - build & push](https://github.com/neon-JS/above_me/actions/workflows/docker-frontend.yml/badge.svg)](https://github.com/neon-JS/above_me/actions/workflows/docker-frontend.yml)

This project is up and running. Currently, there are some bugs and missing features.
It's still in active development as I use it quite frequently.
For concrete TODOs, see [TODO.md](TODO.md).

## License
This code is licensed under the MIT-License (see [LICENSE](LICENSE)). Before using it, make sure to not violate against OGN rules:

see [OGN data usage](https://www.glidernet.org/ogn-data-usage/)  
see [ODbL summary](https://opendatacommons.org/licenses/odbl/summary/)

(This project complies to those rules by only publishing data that's [at most 5 minutes old](backend/src/api/state.rs#L127) and [only for aircraft that don't have stealth- or no-tracking-mode active](backend/src/ogn/aprs/conversion.rs#L26).)