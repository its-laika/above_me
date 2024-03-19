# above_me
What is flying _above\_me_?

## Target
This projects listens to the APRS-servers of the [Open Glider Network](http://wiki.glidernet.org/) and stores all incoming messages.
Via API you can then request a list of gliders near a given position based on the OGN data. 
The next step is a website (or app) which automatically gets your position and returns the list of gliders right above you.

## Configuration
### Backend
There are three ways to configure the backend:
1. _/config.json_
2. _/backend/config.json_ (overrides _1._)
3. Configuration by environment variables with the prefix _ABOVE\_ME\__ (overrides _1._ and _2._)

See [config.example.json](config.example.json) for available config keys.

## Docs
### API
see [openapi.yml](openapi.yml)

## Status
Backend seems to be working and ready.
Frontend hasn't been started yet and will be the next target.

see [TODO.md](TODO.md)

## License
This code is licensed under the MIT-License (see [LICENSE](LICENSE)).
Before using this, make sure to not violate against OGN rules!

- [OGN data usage](https://www.glidernet.org/ogn-data-usage/)
- [ODbL summary](https://opendatacommons.org/licenses/odbl/summary/)
