# DotFiles but with Rust!

## Inspiration

Honestly, maintaining my `.files` was a pain before this change,
bash scripting has always been horrible for me and I'd spend more time
googling how to do simple things (like conditionals and std in/out)
than I would actually modifying my `.files`.

## Yeah I know

This is absolutely unnecessary, but it seemed like a fun idea that
would potentially help me kill my original bash `.files` in favor
of something with better input/output, customization and far more
power while also allowing me to clean up all the junk that was in my
previous configuration.

## The Magic Commands

### Install for the First Time

```sh
curl (TODO)
```

### Updating

If you've installed with `curl` already, you can simply utilize
the small cli that this installs on your machine to manage easy
updating and syncing with my `.files.`

```sh
  .rf update
```

### Syncing

If you don't have permission you're screwed here but for me I can
also sync my .files back to github (oOooo `git` cli proxy) with
`.rf sync`
