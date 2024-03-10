# Forex News Webscraper

A webscraper for forex news written in RUST programming.

SUPPORTED FOREX NEWS SITE
- Forex Factory

## CONFIGURATION

### ENVIRONMENT VARIABLES
- DATABASE_URL - The connection string of the MySQL Database
```
mysql://<database_username>:<database_password>@<database_host>:<database_port>/<database_name>
```

- START_YEAR - The start range of the year to crawl.
```
START_YEAR=<YEAR>
```
- END_YEAR - The end range of the year to crawl.
```
END_YEAR=<YEAR>
```

## DEVELOPMENT

### Generating Entity
```
sea-orm-cli generate entity -v -o src/models

Note: Do not modify the entity files. The cli command will overwrite it.
```

### Database Migration
```
sea-orm-cli migrate up
sea-orm-cli migrate down
```

### Container Database
```
docker-compsoe -f db-docker-compose.yml up -d
```

### TESTING
```
cargo test
```

### RUNNING
```
cargo run
```

## REFERENCES
- Database ORM - https://www.sea-ql.org/SeaORM/docs/index/
- SeaORM Crate - https://docs.rs/sea-orm/0.12.14/sea_orm/index.html
- Headless Chrome - https://docs.rs/headless_chrome/latest/headless_chrome/

## DEVELOPER
- JP Mateo (jpmateo022@gmail.com)

