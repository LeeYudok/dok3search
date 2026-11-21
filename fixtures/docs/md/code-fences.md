# Code fences

Text before the fences.

```sh
# not a heading
## still not a heading
echo done
```

## After backticks

~~~python
# comment, not a heading
def f(): pass
~~~

## Longer fence

````markdown
# Fake title inside a four-backtick fence
```
nested three-backtick fence does not close it
```
## Fake section
````

## Mixed markers

```
# the tilde line below does not close a backtick fence
~~~
# still inside the code
```

## Indented fence

  ```text
  # two spaces of indentation are allowed
  ```

    ```
    # indented code block, so this is not a fence
    ```

## Inline triple backticks

```inline``` at the start of a line is not a fence.

Use ```inline``` triple backticks inside a paragraph.

## End

Last section.
