### next
- fix app name in `--version`
- better error message on path not found - thanks @orhun

<a name="v2.0.0"></a>
### v2.0.0 - 2023-08-30
- `--output` parameter lets you choose between summary tables (default) or the log lines, either raw, as CSV, or as JSON
- `--lines` parameter removed (use `--output raw` or `-o r` instead)
- `--date` precision now the second
- `--time` filter
- new time histogram (time of the day, in the server's timezone)
- more helpful `--help`
- more targets for binaries in the official archives, especially ARM 32/64 both gnu and musl

<a name="v1.7.2"></a>
### v1.7.2 - 2023-04-23
- dependency managment - Fix #22

<a name="v1.7.1"></a>
### v1.7.1 - 2022-06-05
- mostly dependency updates and compilation fixes

<a name="v1.7.0"></a>
### v1.7.0 - 2022-01-16
- allow passing several paths as arguments - Fix #14

<a name="v1.6.0"></a>
### v1.6.0 - 2021-12-25
- better table fitting algorithm, less frequently breaking the histogram columns

<a name="v1.5.5"></a>
### v1.5.5 - 2021-12-21
- don't write an error when no log line matches the query

<a name="v1.5.4"></a>
### v1.5.4 - 2021-11-27
- fix compilation broken by patch release 1.0.49 of anyhow

<a name="v1.5.3"></a>
### v1.5.3 - 2021-07-13
- nothing new visible, small internal upgrades

<a name="v1.5.2"></a>
### v1.5.2 - 2021-06-29
- fix inability to render on narrow terminals

<a name="v1.5.1"></a>
### v1.5.1 - 2021-05-01
- look up to 3 lines of a file for a log line when checking whether it's a log file - Fix #8
- faster log parsing (about 7%)
- IP filtering allow regexes or any string based filtering

<a name="v1.5.0"></a>
### v1.5.0 - 2021-03-19
- new syntax to specify fields, allow adding from default, removing from all, etc. (the old syntax still works)
- compiles on windows (but doesn't know where the log files are) - I need testers to confirm it works
- change error message "no log found" into a more appropriate one when there was an error reading (usually lack of permission)

<a name="v1.4.1"></a>
### v1.4.1 - 2021-03-07
- small details, like the order of arguments in help

<a name="v1.4.0"></a>
### v1.4.0 - 2021-03-03
- `--lines` option to output log lines to stdout
- accept date in ISO 8601 format (previously, only the "common log format" was accepted) - Fix #3

<a name="v1.3.2"></a>
### v1.3.2 - 2021-02-23
- fix wrong version number in rhit.log file
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
