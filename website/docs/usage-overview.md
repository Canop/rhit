
# Launch

`rhit` (or `sudo rhit` depending on your configuration) will open the access logs if they're at their standard location.

You may open a specific file, or a specific directory, by giving the path as argument:

```bash
rhit my/archived/logs
```

# Launch parameters

`rhit --help` will display all available launch arguments.

The arguments you'll most often use enable

* to [filter](../usage-filters) the hits to focus on a specific set: `--date`, `--ip`, `--method`, `--path`, `--referer`, `--status`
* to [choose the displayed fields](../usage-fields): `--fields`
* to [choose the sorting key](../usage-key), either *hits* (default) or *bytes*: `--key`
* to specify the detail level, the length of tables: `--length`, from `0` (short) to `6` (long), `1` being the default
* to see the [recent changes](../usage-changes): `--changes`
* to [export](../export) the filtered lines, either to screen or to a file


