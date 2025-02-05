# Mongoose Export Tool

Simple tool to aquire the URL needed for the Mongoose transaction export.
Fills in the required date period automatically.

## Assumptions

This tool assumes a period runs for the duration of one week, starting on Monday and ending on Sunday.
Currently, this is true in Mollie, the pay-out day is Monday.

Furthermore, it assumes the date range required by Mongoose is non-inclusive on the `period_end` date.
Currently, this assumption holds.

## License

MIT or Apache-2.0, at your option