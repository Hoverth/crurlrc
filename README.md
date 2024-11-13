# CRURLRC

##### clean and reliable URL redirect checker

A program to help test URL redirections.

## Usage

If a redirect to a URL is found, that URL will be printed to stdout,
otherwise it'll just be a blank line. All other output is printed to
stderr.

```text
crurlrc [options] [URLs]

Options
-h --help => show this help
-i        => read URLs from stdin (useful for scripts)
-c        => disable colour output to stderr
```

## License

This project is licensed under the AGPL v3. See `LICENSE` for details.
