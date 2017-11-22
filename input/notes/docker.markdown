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

# Building

A <span class="path">Dockerfile</span> can be built into a Docker image with the `docker build` command. The image is built in a particular context, such as the current directory `.`, and the file named <span class="path">Dockerfile</span> at the root of that context is used by default, unless one is explicitly specified with the `-f` parameter.

Docker images are usually tagged with the `-t` parameter, which can be provided multiple times to facilitate readjusting the `latest` tag, for example.

``` console
$ docker build -t myimage .
```

The instructions in the <span class="path">Dockerfile</span> are run sequentially _and independently_ by the Docker daemon, committing the result of each one to a new image if necessary, then outputting the ID of the resulting image.

Note that since each instruction is run in isolation, a `RUN cd /tmp` command will not affect subsequent instructions.

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
