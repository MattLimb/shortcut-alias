# Templating

Jinja2 Templates will allow more dynamic commands, and easier definition of variables. 

# Contents
- [Templating](#templating)
- [Contents](#contents)
  - [Defining a Template](#defining-a-template)
  - [Using Variables](#using-variables)

## Defining a Template

All fields that can be Jinja2 templates are shown in the `shortcut_file` documentation.

## Using Variables

Variables in Shortcut Alias are split into 4 categories:

1. `args`
2. `variables`
3. `env`
4. `commands`

Each category follows the same basic format for accessing values:

```jinja
{{ <section>.<variable_name> }}
```

### `args`

This section contains the results of the command-line arguments specified in the shortcut file. 

Given this excerpt from a shortcut file:

```yaml
args:
  - name: bin
    arg_type: data
    default: python
    help: The Python binary to invoke.
```

In this case, the `variable_name` we want to use is the `name` key as specified above.

For example:

```jinja
{{ args.bin }}
```

### `variables`

This section copies the `variables` key from the shortcut file, and makes it accessiable using the jinja format. 

For example, this variables section in a shortcut file:

```yaml
variables:
  item: two
```

This would be accessiable like so:

```jinja
{{ variables.item }}
```

### `env`

This section is similar to the `variables` section above.

```yaml
env:
  - SOME_VAR
```

The environemt variable here is accessiable using the following syntax:

```jinja
{{ env.SOME_VAR }}
```

### `commands`

This section is more dynamic than the previous sections. As each command completes it will add an object into this section under the command name.

This object contains the command status and the output of the command. 

Take this command definition:

```yaml
commands:
  - name: pyver
    command: "python -V"
```

Upon completion, the following can be used to get the completed status code:

```jinja
{{ commands.pyver.status }}
```

And the following can be used to get the output of the command:


```jinja
{{ commands.pyver.output }}
```