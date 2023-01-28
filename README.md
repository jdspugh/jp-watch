# jp-watch-rust (depricated, use jp-watch-c instead)

Minimalist app to recursively watch a directory for changes.

Written for ```jp-sync``` because ```fswatch``` and ```inotify-tools``` didn't produce consistent results including directories identified across platforms.

## Limitations

On Linux ```inotify``` is used. Unfortunately ```inotify``` won't watch new files or directories added after the watch has started, unlike the MacOS version which uses ```FSEvents```.

## Installation

```
$ cd ~
$ git clone https://github.com/jdspugh/jp-watch.git
$ cd jp-watch
$ cargo build --release
$ sudo cp target/release/jp-watch /usr/local/bin
```

To uninstall:

```
$ cd ~
$ sudo mv /usr/local/bin/jp-watch /tmp
$ mv jp-watch /tmp
```

## Useage

```
$ jp-watch <directory to watch>
```

Output is of the form:

```
file1.ext1
dir/file2.ext2
dir/
dir/subdir/
```

Note that directories will always end with a slash to it's easy to distinguish them from files.

Examples:

```
$ jp-watch .
```

```
$ jp-watch dir
```
