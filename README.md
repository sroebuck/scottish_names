# Random Scottish Names

This simple Rust library takes lists of birth names in Scotland in 2018 and lists of surnames founds in births, deaths and marriages in 2018 and combines them to form randomised names.

The first names and surnames are individually reproduced randomly but weighted to reflect the frequency of occurrence of the names.  So a common female firstname like ‘Sophie’ will be output more frequently than ‘Aubree’.

The surnames are converted from a capitalised source-list so there may be issues with capitalisation - please raise an issue if you spot one.

Finally, for clarity, this code does not generate names that are traditionally Scottish:  it reflects the multicultural names being given in Scotland.  By using first names given at birth in 2018, the frequency of first names will be biased towards recently popular names rather than reflecting the names across generations that are found in the population.

---

The original names lists came from:

* [Scottish Babies' First Names 2018](https://www.nrscotland.gov.uk/statistics-and-data/statistics/statistics-by-theme/vital-events/names/babies-first-names/babies-first-names-2018/babies-first-names-2018)
* [Common Scottish Surnames in Birth, Marriage and Death registers](https://www.nrscotland.gov.uk/statistics-and-data/statistics/statistics-by-theme/vital-events/names/most-common-surnames)

## Release notes

### Version 0.1.0

First release

### Version 0.1.1

Fix to API documentation.

### Version 0.2.0

Breaking change with surnames now in mixed case rather than capitalised.

### Version 0.2.1

Added link to this `README.md` in `Cargo.toml` so that it would appear in crates.io.