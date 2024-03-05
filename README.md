# Datesorter

A small utility to sort input data by ISO 8601 date.

## Build

```bash
cargo build --release
```

## Usage

Given an input like this:

```verbatim
Lorem ipsum dolor sit amet, 2023-09-05T08:20:15Z, consectetur adipiscing elit
Fusce euismod justo nec, 2024-05-18T14:45:30Z, ultricies semper
Nulla facilisi, 2022-12-10T11:10:25Z, aliquam euismod
Sed vitae mauris, 2023-02-28T17:35:40Z, vestibulum
Proin auctor velit, 2024-07-15T09:50:55Z, eget semper
Nunc id semper, 2022-11-20T12:15:10Z, consequat
Etiam auctor felis, 2023-04-03T19:30:25Z, in tristique
```

You can sort it by date with:

```bash
datesorter -d "," -c 1 --sort desc input.txt
```

This will output:

```verbatim
Proin auctor velit,2024-07-15T09:50:55Z,eget semper
Fusce euismod justo nec,2024-05-18T14:45:30Z,ultricies semper
Lorem ipsum dolor sit amet,2023-09-05T08:20:15Z,consectetur adipiscing elit
Etiam auctor felis,2023-04-03T19:30:25Z,in tristique
Sed vitae mauris,2023-02-28T17:35:40Z,vestibulum
Nulla facilisi,2022-12-10T11:10:25Z,aliquam euismod
Nunc id semper,2022-11-20T12:15:10Z,consequat
```
