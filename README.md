# above_me
What is flying _above\_me_?

## Goal
This projects listens to the APRS-servers of the [Open Glider Network](http://wiki.glidernet.org/) and stores all incoming messages internally. Via an API you can request a list of gliders near a given position based on the OGN data. A small website, which automatically fetches your position, returns the list of gliders right above you.

## Configuration
### Native
There are three ways to configure _above\_me_:
1. _/config.json_
2. _/backend/config.json_ (overrides _1._)
3. by environment variables with the prefix _ABOVE\_ME\_\__ (overrides _1._ and _2._)

See [config.example.json](config.example.json)¹ ² and [/docker/.env.example](docker/.env.example)³ for available options.

### Docker
Configure by setting up a _/docker/.env_ file and run `docker compose up`.

See [/docker/.env.example](docker/.env.example) for available options.  

(Please note that the _bind\_url_ should be left unconfigured as it is part of the [/docker/docker-compose.yml](docker-compose.yml) config file. Otherwise the proxy-pass may break.)

### Privacy policy
The website contains links to _privacy-policy.html_. You can (and should!) set up this privacy policy page. Empty dummy files already exist in the _/docker_ and _/frontend/src_ directories.

## API
see [openapi.yml](openapi.yml)

## Status
see [TODO.md](TODO.md)

## License
This code is licensed under the MIT-License (see [LICENSE](LICENSE)). Before using _above\_me_, make sure to not violate against OGN rules:

see [OGN data usage](https://www.glidernet.org/ogn-data-usage/)  
see [ODbL summary](https://opendatacommons.org/licenses/odbl/summary/)
