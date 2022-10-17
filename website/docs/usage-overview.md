
# Log files

`rhit` (or `sudo rhit` depending on your configuration) will open the access logs if they're at their standard location.

You may open a specific file, or a specific directory, by giving the path as argument:

```bash
rhit my/archived/logs
```

# Launch parameters

`rhit --help` will display all available launch arguments.

The arguments you'll most often use enable

* [filtering](../usage-filters) hits to focus on a specific subset: `--date`, `--ip`, `--method`, `--path`, `--referer`, `--status`
* [choosing the displayed fields](../usage-fields): `--fields`
* [choosing the sorting key](../usage-key), either *hits* (default) or *bytes*: `--key`
* specifying the detail level, the length of tables: `--length`, from `0` (short) to `6` (long), `1` being the default
* seeing the [recent changes](../usage-changes): `--changes`
* [exporting](../export) the filtered lines, either to screen or to a file


