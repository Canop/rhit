### next
- fix wrong version number in log file
- any file whose name contains "access.log" is considered a probable log file
- when a single file is given to rhit, its name isn't checked
- no file name is checked with `--no-name-check`

<a name="v1.3.1"></a>
### v1.3.1 - 2021-02-19
- `--all` argument to remove the filter excluding "resources" from the paths tables

<a name="v1.3.0"></a>
### v1.3.0 - 2021-02-18
Many changes in the arguments you give to rhit:
- `tables` have been renamed `fields`
- `addr` (remote IP addresses) has been changed to `ip` both in fields list and as filter
- instead of a `trends` table, there's a `--changes` argument (short: `-c`)
- with `--changes`, you see more popular and less popular referers
- with `--changes`, you see more popular and less popular remote ip adresses if the ip field is shown (eg with `rhit -f date,ip -c`)
- date filters can be negative or inequalities (eg: `-d '>2021/02/10'`)

<a name="v1.2.0"></a>
### v1.2.0 - 2021-02-12
- the `--key` argument defines the key measure, either 'hits' (default) or 'bytes' (of the response) used for sorting and filtering, and highlighted in pink
- you can filter on year or month (eg `rhit -d 2021/02`)
- trends in all tables

<a name="v1.1.1"></a>
### v1.1.1 - 2021-02-10
- when you pipe the output of rhit to a file, there's no style information. You can choose explicitly to have or not the styles and colors with the `--color` argument - Fix #1

<a name="v1.1.0"></a>
### v1.1.0 - 2021-02-09
- trends

<a name="v1.0.0"></a>
### v1.0.0 - 2021-01-29
- first public release
