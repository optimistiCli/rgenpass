# Password generator

Generates random password(s). Default is 16 chars of capital, lowercase letters and digits.

## Usage
```
genpass [-h] [-l | -L] [-u | -U] [-d | -D] [-s | -S | -r <chars> | -x <chars>]
        [-a] [-n <num>] [-q] [length]
```

## Options And Arguments
* `-l` – Use lower case letters (default)
* `-L` – Do NOT use lower case letters
* `-u` – Use upper case letters (default)
* `-U` – Do NOT use upper case letters
* `-d` – Use digits (default)
* `-D` – Do NOT use digits
* `-s` – Use special characters `!$%@#`
* `-S` – Do NOT use special characters
* `-x <chars>` – Extra special characters, implies -s
* `-r <chars>` – Replacement special characters, implies -s
* `-a` – Password must conain chars of all classes
* `-n <num>` – Generate number of passwords, 1 if ommited
* `-q` – Quiet output, doesn't affect anithing as of yet
* `[length]` – Password length, 16 if ommited.

## History
It is a rust rewrite of an old [shell script](https://github.com/optimistiCli/genpass).
