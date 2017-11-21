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

A <span class="path">.dockerignore</span> file can be used exclude files and directories from a resulting image, similar to <span class="path">.gitignore</span>.

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
