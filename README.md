# EuroRust Derive Example Project

This is an example project that creates a derive helper for working with AWS SQS.

Implementing the queue was done during a live coding session at EuroRust 2024 (Vienna).

## Notes

- The application works with localstack, see the `localstack` dir
- While it probably makes more sense for a derive to add the `send` and `receive` to the struct itself, creating a new struct allowed me to show how to create a new struct / ident
- As mentioned during the presentation, for this example it would probably have been better to just use generic functions instead of turning to macros
