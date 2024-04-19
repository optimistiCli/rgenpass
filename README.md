# Password generator

```
Usage: genpass [<length>] [-l] [-L] [-u] [-U] [-d] [-D] [-s] [-S]
               [-x <extra...>] [-r <replacement>] [-a] [-n <num>] [-q]

Generates random password(s). Default is 16 chars of capital, lowercase
letters and digits.

Positional Arguments:
  length

Options:
  -l, --lower       use lower case letters (default)
  -L, --no-lower    do NOT use lower case letters
  -u, --upper       use upper case letters (default)
  -U, --no-upper    do NOT use upper case letters
  -d, --digits      use digits (default)
  -D, --no-digits   do NOT use digits
  -s, --special     use special characters !$%@#
  -S, --no-special  do NOT use special characters (default)
  -x, --extra       extra special characters, implies -s
  -r, --replacement replacement special characters, implies -s
  -a, --all         password must conain chars of all classes
  -n, --num         generate number of passwords (default is 1)
  -q, --quiet       do not output anything but the password(s)
  -h, --help        display usage information
```

