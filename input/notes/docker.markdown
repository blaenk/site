---
title = "Docker"
published = "November 20, 2017"
excerpt = "Containers"
comments = false
---

Docker is a container infrastructure project facilitating the creation and execution of containers through the use of resource isolation features such as Linux' cgroups and kernel namespaces.

<toc />

# Terminology

An _image_ is a self-contained executable package. A _container_ is an instance of an image. The distinction is similar to an executable binary and process instances of it, except that an image is generally self-contained so that it doesn't depend on system-wide dependencies, for example, and unlike a process, a container runs in a restricted environment [^security] separate from the host system.

[^security]: Note that this restricted environment doesn't necessarily imply security. A container is not necessarily a secure sandbox.

The advantage of containers is that they're much lighter-weight and "native" than traditional [virtualization], which incurs hardware virtualization, the [hypervisor], and the operating system and its necessary processes.

[virtualization]: /notes/linux/#virtualization
[hypervisor]: /notes/linux/#hypervisor-based-virtualization

# Dockerfile

A <span class="path">Dockerfile</span> describes how to construct the image, the image dependencies that it should build on top of, ports and file systems that the host can map into, environment variables to define, and the command to run when the image is instantiated as a container.

``` dockerfile
# Use an official Python runtime as a parent image
FROM python:2.7-slim

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
ADD . /app

# Install any needed packages specified in requirements.txt
RUN pip install --trusted-host pypi.python.org -r requirements.txt

# Make port 80 available to the world outside this container
EXPOSE 80

# Define environment variable
ENV NAME World

# Run app.py when the container launches
CMD ["python", "app.py"]
```

A <span class="path">Dockerfile</span> must begin with a `FROM` instruction indicating the base image from which the image builds. Specifically, `FROM` can only be preceded by one or more `ARG` instructions which themselves declare arguments used in the `FROM` lines.

Parser directives take the form `# directive=value` and must appear at the very top of the <span class="path">Dockerfile</span>.

Environment variables can be expanded by referring to them as `$env_var` or `${env_var}`, with the latter format supporting certain standard Bash modifiers:

* `${var:-word}` indicates to use `word` as the value if the variable isn't set.
* `${var:+word}` indicates to use `word` as the value if the variable is set.

Note that `word` in the previous examples can itself be another environment variable.

A <span class="path">.dockerignore</span> file can be used exclude files and directories from a resulting image, similar to <span class="path">.gitignore</span>. This file can be made into a whitelist instead of a blacklist by starting with a `*` rule which excludes everything, then adding exceptions with the `!` prefix.

## Best Practices

Remember that most instructions add a new layer to the image, so it's necessary to cleanup artifacts that aren't needed before moving to the next layer.

## Instructions

### FROM

``` dockerfile
FROM <image> [AS <name>]
FROM <image>[:<tag>] [AS <name>]
FROM <image>[@<digest>] [AS <name>]
```

The `FROM` instruction initializes a new build stage, setting the base image for subsequent instructions. It can appear multiple times in a single <span class="path">Dockerfile</span> to create multiple images or use one build stage as a dependency for another. Each `FROM` instruction clears state created by previous instructions.

A build stage can be named by adding `AS name` to the `FROM` instruction, then the image built from that stage can be referred to by name in subsequent `FROM` or `COPY --from` instructions.

The `ARG` instruction can be used before any `FROM` instructions to declare variables that can be expanded and used by `FROM` instructions. Note that an `ARG` is outside of a build stage, so it can't be used within one, i.e. after a `FROM`, unless explicitly requested by using the `ARG` instruction to refer to it.

``` dockerfile
ARG CODE_VERSION=latest
FROM base:${CODE_VERSION}
CMD /code/run-app

FROM extras:${CODE_VERSION}
CMD /code/run-extras

FROM busybox:${CODE_VERSION}
# Explicitly declare it within the build stage
ARG CODE_VERSION
RUN echo ${CODE_VERSION} > image_version
```

### RUN

``` dockerfile
RUN <command>
RUN ["executable", "param1", "param2"]
```

The `RUN` instruction executes commands in a new layer on top of the current image, then commits the results, resulting in a committed image used for the next step.

Commands can be executed in an `exec()` form with an explicit array or with a `system()` shell form, which uses `sh -c` on Linux by default and `cmd /S /C` on Windows. The default shell can be changed with the `SHELL` command. The `exec()` form takes a JSON array, so elements require double quotes `"`.

A line continuation `\` can be used to continue a single `RUN` instruction, causing multiple commands to contribute to a single committed image, as opposed to generating a separate image for each command in the `RUN` as would be the case if a separate `RUN` instruction were specified for each command.

When used to `apt-get update`, it should be combined with `apt-get install` in the same `RUN` statement to avoid [caching issues], since the `apt-get update` command would not have changed even if subsequent `apt-get install` commands did. This is a form of cache-busting, which can also be done by version pinning.

[caching issues]: #image-cache

Commands that use pipes and the `system()` shell form are considered successful as long as the final operation succeeds, regardless of the status of preceding operations. This can be overridden by prefixing `set -o pipefial &&` to the command.

### CMD

``` dockerfile
# Exec form
CMD ["executable","param1","param2"]

# Shell form
CMD command param1 param2

# Default arguments for ENTRYPOINT
CMD ["param1","param2"]
```

The `CMD` instruction's main purpose is to provide defaults for the container's executable and/or its arguments. When an executable is specified, it serves to set the command to be executed when running the image. If no executable is specified, then the image will also require an `ENTRYPOINT` instruction which does specify an executable. Any arguments specified by `CMD` are _defaults_ and are automatically overridden when the user specifies any arguments to `docker run`.

There can only be one `CMD` instruction in a <span class="path">Dockerfile</span>, so only the final `CMD` instruction will take effect.

Like `RUN`, the `CMD` instruction allows both `exec()` and `system()` forms.

### LABEL

``` dockerfile
LABEL <key>=<value> <key>=<value> <key>=<value> …
```

The `LABEL` instruction adds key-value metadata pairs to an image, in a format similar to environment variables. Double quotes `"` can be used to add spaces, and backslash `\` can be used as line continuations to add newlines.

``` dockerfile
LABEL "com.example.vendor"="ACME Incorporated"
LABEL com.example.label-with-value="foo"
LABEL version="1.0"
LABEL description="This text illustrates \
that label-values can span multiple lines."
```

It used to be conventional to combine multiple labels into a single `LABEL` instruction, since each `LABEL` instruction resulted in a new layer prior to Docker 1.10.

An image's labels can be viewed with the `docker inspect` command.

### EXPOSE

``` dockerfile
EXPOSE <port> [<port>/<protocol>…]
```

The `EXPOSE` function doesn't actually publish a port, but rather serves as documentation and to inform Docker that the container listens on the specified ports. If no protocol is specified, TCP is assumed.

``` dockerfile
# Assume TCP
EXPOSE 80

EXPOSE 127/UDP
```

Ports are actually published once the container is run with the `-p` argument to the `docker run` command, or the `-P` argument to publish all exposed ports and map them to higher-order ports.

### ENV

``` dockerfile
ENV <key> <value>
ENV <key>=<value> …
```

The `ENV` instruction sets an environment variable for use by all descendant instructions.

One form uses a space to separate the key from the value and only supports a single key-value pair, while the second form uses an equal sign `=` to separate keys from values and so supports multiple key-value pairs in one instruction.

An image's environment variables can be viewed with the `docker inspect` command and overridden with the `--env` argument to the `docker run` command.

Note that it's also possible to set an environment variable for the duration of a `RUN` command through the shell's own functionality.

The `ENV` instruction can be used to update the `PATH` environment variable, to facilitate for example:

``` dockerfile
ENV PATH /usr/local/nginx/bin:${PATH}
CMD ["nginx"]
```

### ADD

``` dockerfile
ADD <src>… <dest>
ADD ["<src>", … "<dest>"]
```

The `ADD` instruction copies files, directories, or remote file URLs from a source to a destination on the image's filesystem. Each source may contain patterns. The destination path is either absolute or relative to the `WORKDIR`.

All new files and directories are created with a UID and GID or `0`.

### COPY

``` dockerfile
COPY <src>… <dest>
COPY ["<src>", … "<dest>"]
```

The `COPY` instruction copies files or directories from the specified sources to the specified destination path on the container's filesystem. Each source may contain filename patterns. The destination path is either absolute or relative to the `WORKDIR`.

If the destination doesn't exist, it is created along with any directories along its path.

If a source is a directory, all of its contents (not the directory itself) are copied including the filesystem metadata.

Sources must be descendants of the build's context. It's not possible to `COPY` something from an ancestor.

All new files and directories are created with a UID and GID or `0`.

`COPY` accepts an optional `--from=name|index` argument that specifies a previous build stage from which to source the applies, either by specifying the name given to the build stage with `AS <name>` or explicitly specifying the build stage index. If a build stage with the specified name isn't found, an image with the same name is attempted instead.

Carefully choose the granularity of `COPY` instructions so that the cache is only invalidated for certain files.

### ENTRYPOINT

``` dockerfile
ENTRYPOINT ["executable", "param1", "param2"]
ENTRYPOINT command param1 param2
```

The `ENTRYPOINT` instruction can be used to specify the command to run when the container is executed. Additional arguments given to the `docker run <image>` command are appended after _all_ elements of the `exec()` form, while the `system()` form completely prevents `CMD` or `docker run` arguments from being applied.

Note that the shell form executes the command as a subcommand `sh -c`, so the specified executable will _not_ be the container's PID 1 and will not receive Unix signals, so will not receive the `SIGTERM` signal from the `docker stop` command. One way to remedy this is to run it with `exec`.

The `ENTRYPOINT` instruction can be overridden with the `--entrypoint` argument to the `docker run` command, but only to set the binary to `exec()`.

Only the final `ENTRYPOINT` instruction takes effect.

It may be useful to specify common "base" arguments through `ENTRYPOINT` and define additional arguments through `CMD`, which are more likely to be changed, since arguments to `docker run` _append_ to those specified in `ENTRYPOINT` whereas they completely replace those specified in `CMD`.

### VOLUME

``` dockerfile
VOLUME ["/data", …]
VOLUME /var/log /var/db …
```

The `VOLUME` instruction creates a mount point and marks it as holding externally mounted volumes from the native host or other containers. The host directory is specified at run-time.

Any data that already exists at the specified location is used to initialize the newly created volume. However, if any build steps change the data within the volume _after_ it has been declared, the changes are discarded.

### USER

``` dockerfile
USER <user>[:<group>] or
USER <UID>[:<GID>]
```

The `USER` instruction sets the user name (or UID) and optionally the user group (or GID) to use when running the image and any `RUN`, `CMD`, and `ENTRYPOINT` instructions that follow it.

Note that if the user has not primary group then the `root` group is assumed.

If root privileges aren't required, a user and group should be created and used. If privileges are required, use [`gosu`].

``` dockerfile
RUN groupadd -r postgres && useradd --no-log-init -r -g postgres postgres
```

[`gosu`]: https://github.com/tianon/gosu

### WORKDIR

``` dockerfile
WORKDIR /absolute/path
WORKDIR relative/to/previous/workdir
```

The `WORKDIR` instruction sets the current directory for any `RUN`, `CMD`, `ENTRYPOINT`, `COPY`, and `ADD` instructions that follow. It is created if it doesn't exist, even if it's not used in any subsequent instructions.

Environment variables can be expanded but only those explicitly set with the `ENV` instruction.

Relative paths are relative to the previously-set `WORKDIR`, which can be confusing and difficult to track, so absolute paths are preferred.

### ARG

``` dockerfile
ARG <name>[=<default value>]
```

The `ARG` instruction can define a variable that users pass at build-time with the `--build-arg <name>=<value>` argument to the `docker build` command.

Note that these build-time variables should not be used for secrets since they are visible to any user of the image with the `docker history` command.

The cache is invalidated when a build-time variable is given a different value from the previous build. All `RUN` instructions have an implicit dependency on preceding `ARG` variables, which may trigger cache invalidation.

Environment variables defined with the `ENV` instruction always override an `ARG` declaration of the same name.

An `ARG` declaration is scoped to its current build stage, and so goes out of scope at the end of it.

The `ARG` instruction can be viewed as a directive to bring the command-line argument into scope, with an optional default value in case there is no command-line argument. This explains why the variable is empty unless the `ARG` declaration exists for it in the current scope, and why each build stage that requires access to the variable needs to explicitly declare it by repeating the `ARG` declaration.

``` dockerfile
FROM busybox
ARG SETTINGS
RUN ./run/setup ${SETTINGS}

FROM busybox
ARG SETTINGS
RUN ./run/other ${SETTINGS}
```

Since environment variables are always persisted into the image, a build-time variable can be used to allow build-time overriding of environment variables that are persisted into the image.

``` dockerfile
FROM ubuntu

# Allow build-time overriding with --build-arg=
ARG CONT_IMG_VER

# Create environment variable of the same name, seeded with the build-time
# variable if any exists, or the default value "v1.0.0"
ENV CONT_IMG_VER ${CONT_IMG_VER:-v1.0.0}

RUN echo $CONT_IMG_VER
```

Docker pre-defines a certain number of build-time variables, mainly relating to proxies, such as `HTTP_PROXY`.

### ONBUILD

``` dockerfile
ONBUILD [INSTRUCTION]
```

The `ONBUILD` instruction can register any another instruction to trigger and execute when the image is used as a base for another build. The trigger executes in the context of the downstream build which is using this image as a base, as if it had been inserted immediately after the `FROM` instruction which references this image.

### STOPSIGNAL

``` dockerfile
STOPSIGNAL signal
```

The `STOPSIGNAL` instruction is used to specify the signal to send to the container to have it exit. This can be a number corresponding to the signal or the signal name such as `SIGKILL`.

### HEALTHCHECK

``` dockerfile
HEALTHCHECK [OPTIONS] CMD <command>

# Disable any inherited health checks
HEALTHCHECK NONE
```

The `HEALTHCHECK` instruction tells Docker how to check that the container is still working. The command can be in `exec()` or `system()` form. The expected exit status of the health check command are:

0. success: the container is healthy and usable
1. unhealthy: the container isn't working correctly
2. reserved; don't use this exit code

The command's stdout and stderr are stored in the health status which can be queried with the `docker inspect` command.

Containers with a health check specified gains a health status which begins as `starting` and transitions to `healthy` whenever a health check passes. After a certain number of consecutive failures it becomes `unhealthy`.

The `HEALTHCHECK` instruction allows a set of options:

| Option           | Default |
| :--------------- | :------ |
| `--interval`     | `30s`   |
| `--timeout`      | `30s`   |
| `--start-period` | `0s`    |
| `--retries`      | `3`     |

The first health check runs `--interval` seconds after the container is started and on every such interval after a health check completes.

A health check is considered failed if it takes longer than `--timeout` seconds to complete.

A container is considered `unhealthy` if it suffers `--retries` consecutive failures.

The `--start-period` is a grace period from the container's time of initialization to allow it time to fully start up, during which health check failures are not counted toward the maximum number of retries, until the first health check success.

Only the final `HEALTHCHECK` takes effect.

### SHELL

The `SHELL` instruction can be used to specify the shell command to use for the `system()` form of commands. The default on Linux is `["/bin/sh", "-c"]` and on Windows is `["cmd", "/S", "/C"]`.

The `SHELL` instruction can appear multiple times in the same <span class="path">Dockerfile</span> and affects all subsequent instructions.

# Building

A <span class="path">Dockerfile</span> can be built into a Docker image with the `docker build` command. The image is built in a particular context, such as the current directory `.`, and the file named <span class="path">Dockerfile</span> at the root of that context is used by default, unless one is explicitly specified with the `-f` parameter.

Docker images are usually tagged with the `-t` parameter, which can be provided multiple times to facilitate readjusting the `latest` tag, for example.

``` console
$ docker build -t myimage .
```

The instructions in the <span class="path">Dockerfile</span> are run sequentially _and independently_ by the Docker daemon, committing the result of each one to a new image if necessary, then outputting the ID of the resulting image.

Note that since each instruction is run in isolation, a `RUN cd /tmp` command will not affect subsequent instructions.

## Image Cache

Each instruction is examined by Docker to determine if an existing image in the cache can be reused, in the following manner:

1. Starting with the cached parent image, compare the next instruction against all images derived from that base image to see if any of them was built with the exact same instruction. Otherwise invalidate the cache.
2. For `ADD` and `COPY`, checksums are calculated for all of the files and the cache is invalidated if there are any discrepancies.
3. Cache checking doesn't check files from any other commands, such as `RUN apt-get -y update`, instead the command string itself is compared.

On cache invalidation, all subsequent commands generate new images.

# Tags

Images can be tagged at build-time with the `-t` parameter to `build` or by using the `docker tag` command.

Tag names generally consist of three components: repository, name, and version.

```
repository/name:version
```

For example:

```
fedora/httpd:version1.0
```

When the `:version` component is missing, most Docker commands assume `:latest`.

In order to push an image to a private repository (i.e. not the central Docker registry), it must be tagged with the registry hostname and port.

``` text
myregistry.com:5000/fedora/httpd:version1.0
```

Tags for associating a local image with a repository on a registry follow a similar format:

``` text
username/repository:tag
```

# Execution

A container can be run with the `docker run` command. The `-p` parameter can be used to map a host port into an exposed container port.

``` console
$ docker run -p 4000:80 myimage
```

The container is run in the foreground unless the `-d` parameter is passed to instruct it to run in detached mode.

All containers running on the host can be listed with the `docker container ls` command.

A container can be stopped with the `docker container stop :id` command.

An image on a remote repository can be run by simply referring to it by its remote tag name, and Docker automatically handles obtaining any locally-missing images.

``` console
$ docker run -p 4000:80 user/repo:tag
```

By default, a container's hostname is set to the container's ID.

# Publishing

Images can be published to a registry, such as the default Docker registry.

Images should be tagged with the `docker tag` command with the `user/repo:tag` format.

``` console
$ docker tag myimage blaenk/get-started:part2
```

The images are then pushed with the docker push command.

``` console
$ docker push blaenk/get-started:part2
```

# Management

Active processes can be listed with the `docker ps` command.

Images in the local Docker image registry can be listed with the `docker images` command.
