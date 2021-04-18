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

Most of the uses of templated fields are to retrieve values of variables. 

To retrieve a variable:

```yaml
"{{ <major_key>.<group>.<variable> }}"
```

Unfortunately, due to the the workings of YAML, the double quotes `"` is required either side of the declaration `{{ }}` otherwise there will be some YAML parsing issues. 