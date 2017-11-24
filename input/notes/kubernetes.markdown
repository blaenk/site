---
title = "Kubernetes"
published = "November 23, 2017"
excerpt = "Container orchestration"
comments = false
---

Kubernetes automates the distribution and scheduling of application containers across a cluster in an efficient way. [Docker] itself is adding support for Kubernetes as well.

[Docker]: /notes/docker

<toc />

# Clusters

A cluster consists of a _master_ that coordinates the cluster and _Nodes_ that run applications. A cluster that handles production traffic should have at least three Nodes.

The master coordinates all activities in a cluster, such as scheduling applications, maintaining their desired state, scaling them, and rolling out new updates.

A Node serves as a worker machine in the cluster. Each Node has a Kubelet, which is an agent for managing the Node and communicating with the master.

![cluster](https://d33wubrfki0l68.cloudfront.net/99d9808dcbf2880a996ed50d308a186b5900cec9/40b94/docs/tutorials/kubernetes-basics/public/images/module_01_cluster.svg)

When deploying applications, the master is told to start the application containers, so the master schedules the containers to run on the cluster's Nodes. The Nodes communicate with the master using the Kubernetes API exposed by the master.

# Deployments

A Deployment is responsible for creating and updating instances in an application.

A _Deployment Configuration_ instructs Kubernetes on how to create and update instances of an application. The master schedules application instances onto individual Nodes on the cluster. Once application instances are created, a _Deployment Controller_ continuously monitors them. If a Node hosting an instance goes down or is deleted, the Deployment Controller replaces it, providing a self-healing mechanism to address machine failure or maintenance.

![deployment](https://d33wubrfki0l68.cloudfront.net/152c845f25df8e69dd24dd7b0836a289747e258a/4a1d2/docs/tutorials/kubernetes-basics/public/images/module_02_first_app.svg)

Deployments can be created and managed with the `kubectl` command.

## Deployment Definitions

A Deployment object defines a Pod creation template and desired replica count. The Pods it manages are selected using a label selector, and it creates or deletes Pods as needed to meet the desired replica count.

``` yaml
apiVersion: apps/v1beta1
kind: Deployment
metadata:
  name: nginx-deployment
spec:
  # Run 2 Pods matching the template.
  replicas: 2
  # Create Pods using Pod definition in this template.
  template:
    metadata:
      # Unlike in Pod definitions, no name is necessary
      # because a unique name is generated from the deployment
      # name.
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:1.7.9
        ports:
        - containerPort: 80
```

This deployment can be created with:

``` console
$ kubectl create -f ./deployment.yaml
```

Changes can be applied by using the `apply` sub-command:

``` console
$ kubectl apply -f ./deployment-update.yaml
```

# Pods

A Pod is a group of one or more application containers. It includes shared storage (volumes), IP addresses, and information about how to run them.

Kubernetes creates Pods to host an application instance from Deployment Configurations. A _Pod_ is an abstraction that represents a group of one or more application containers and some shared resources for those containers which may include:

* Volumes: shared storage
* Networking: unique cluster of IP addresses
* Information about how to run each container, such as image version or ports to use

A Pod models an application-specific "logical host" that can contain different application containers which are relatively tightly coupled, such as a Node.js app and a container that feeds the data to be published by the app.

A Pod is the atomic unit in Kubernetes. Deployments create Pods with containers inside them, not containers directly. Pods are tied to the Node onto which they were scheduled and remain there until termination, depending on the restart policy, or deletion.

Containers in a Pod share an IP address and port space, they're always co-located and co-scheduled, and they run in a shared context on the same Node.

Containers should only be scheduled together in a single Pod if they're tightly coupled and need to share resources, such as a disk.

![pods](https://d33wubrfki0l68.cloudfront.net/fe03f68d8ede9815184852ca2a4fd30325e5d15a/98064/docs/tutorials/kubernetes-basics/public/images/module_03_pods.svg)

A [PodPreset] is an object that can be used to [inject][podpreset-injection] information like secrets, volume mounts, and environment variables into Pods at creation time.

[PodPreset]: https://kubernetes.io/docs/concepts/workloads/pods/podpreset/
[podpreset-injection]: https://kubernetes.io/docs/tasks/inject-data-application/podpreset/

## Pod Definitions

A Pod definition declares the _desired state_ which Kubernetes aims to match.

A Pod definition for an nginx web server may look like this:

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
spec:
  containers:
  - name: nginx
    image: nginx:1.7.9
    ports:
    - containerPort: 80
```

This Pod can then be created:

``` console
$ kubectl create -f ./pod-nginx.yaml
```

Volumes can be defined with the `volumes:` section, and mounted with the `volumeMounts:` section. Volume types include `EmptyDir` which creates a new directory that is tied to the Pod's lifetime but is otherwise persisted across failures and restarts, or `HostPath` which mounts an existing directory on the node's file system. See this example for a Redis Pod:

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: redis
spec:
  containers:
  - name: redis
    image: redis
    volumeMounts:
    - name: redis-persistent-storage
      mountPath: /data/redis
  volumes:
  - name: redis-persistent-storage
    emptyDir: {}
```

# Nodes

A Node is a worker machine in Kubernetes. Multiple Pods can run on a single Node.

A Node can have multiple Pods. The Kubernetes master automatically handles scheduling Pods across the Nodes in the cluster based on the available resources on each Node. Each Node runs at least:

* Kubelet: a process responsible for communicating between the master and the Nodes in the cluster. It manages the Pods and the containers running on the machine.
* Container runtime: Such as Docker, responsible for pulling the container image from a registry, then unpacking and running it.

![node](https://d33wubrfki0l68.cloudfront.net/5cb72d407cbe2755e581b6de757e0d81760d5b86/a9df9/docs/tutorials/kubernetes-basics/public/images/module_03_nodes.svg)

# Services

A Service is an abstraction layer which defines a logical set of Pods and enables external traffic exposure, load balancing, and Service discovery for those Pods. They provide a way to refer to a set of Pods, selected by labels, with a single static IP address.

Pods have a lifecycle. When a Node dies, the Pods running on it are lost. A _Replication Controller_ may dynamically drive the cluster back to the desired state by creating new Pods to keep the application running.

Each Pod in a cluster has a unique IP address, even those on the same Node, so there needs to be a way of automatically reconciling changes among Pods.

A _Service_ is an abstraction that defines a logical set of Pods and a policy by which to access them. Services enable loose coupling between dependent Pods. A Service can be defined using YAML or JSON, like all Kubernetes objects. The Pods targeted by a Service are usually determined by a _LabelSelector_.

![services](https://d33wubrfki0l68.cloudfront.net/cc38b0f3c0fd94e66495e3a4198f2096cdecd3d5/ace10/docs/tutorials/kubernetes-basics/public/images/module_04_services.svg)

The different unique IP addresses of each Pod are not exposed outside of the cluster without a Service. There are different types of Services which can be specified via `type` in the ServiceSpec:

* `ClusterIP` (default): Exposes the Service on an internal IP in the cluster, making it reachable only from within the cluster.
* `NodePort`: Exposes the Service on the same port of each selected Node in the cluster using NAT, making it accessible from outside the cluster using `ip:port`. This is a supserset of `ClusterIP`.
* `LoadBalancer`: Creates an external load balanced in the current cloud (if supported) and assigns a fixed external IP address to the Service. Superset of `NodePort`.
* `ExternalName`: Exposes the Service using an arbitrary name as specified by `externalName` by returning a CNAME record with the name. No proxy is used.

A Service may not define `selector` in the spec, which means the corresponding Endpoints object isn't created, allowing manually mapping a Service to specific endpoints.

Services handle discovery and routing among dependent Pods. Services match the set of dependent Pods using labels and selectors, a grouping primitive that allows logical operation on objects in Kubernetes. Labels are key-value pairs attached to objects that can be used to:

* designate objects for development, test, and production
* embed version tags
* classify an object using tags

Labels can be attached at object creation or later, and can be modified at any time.

![labels](https://d33wubrfki0l68.cloudfront.net/b964c59cdc1979dd4e1904c25f43745564ef6bee/f3351/docs/tutorials/kubernetes-basics/public/images/module_04_labels.svg)

Note that Services can be created when a Deployment is created by using the `--expose` argument with `kubectl`.

A Service can handle scaling while remaining accessible in the same manner. Services have an integrated load-balancer that distributes network traffic to all Pods of an exposed Deployment. Services continuously monitor the running Pods' endpoints to ensure that the traffic is only sent to available Pods.

![service-pre-scale](https://d33wubrfki0l68.cloudfront.net/043eb67914e9474e30a303553d5a4c6c7301f378/0d8f6/docs/tutorials/kubernetes-basics/public/images/module_05_scaling1.svg)

![service-post-scale](https://d33wubrfki0l68.cloudfront.net/30f75140a581110443397192d70a4cdb37df7bfc/b5f56/docs/tutorials/kubernetes-basics/public/images/module_05_scaling2.svg)

Rolling Updates allow Deployments' updates to occur with zero downtime by incrementally updating Pod instances, then scheduling those new Pods on Nodes with available resources. By default, only one Pod can be unavailable and only one new Pod can be created, that is, Pods are updated one by one.

In Kubernetes, updates are versioned and any Deployment update can be reverted to a previous version.

The Service will load-balance traffic only to available Pods during the update. Rolling updates allow:

* Promote an application from one environment to another, via container image updates.
* Rollback to previous versions.
* Continuous Integration and Continuous Delivery of applications with zero downtime.

![rolling-update-1](https://d33wubrfki0l68.cloudfront.net/30f75140a581110443397192d70a4cdb37df7bfc/fa906/docs/tutorials/kubernetes-basics/public/images/module_06_rollingupdates1.svg)

![rolling-update-2](https://d33wubrfki0l68.cloudfront.net/678bcc3281bfcc588e87c73ffdc73c7a8380aca9/703a2/docs/tutorials/kubernetes-basics/public/images/module_06_rollingupdates2.svg)

![rolling-update-3](https://d33wubrfki0l68.cloudfront.net/9b57c000ea41aca21842da9e1d596cf22f1b9561/91786/docs/tutorials/kubernetes-basics/public/images/module_06_rollingupdates3.svg)

![rolling-update-4](https://d33wubrfki0l68.cloudfront.net/6d8bc1ebb4dc67051242bc828d3ae849dbeedb93/fbfa8/docs/tutorials/kubernetes-basics/public/images/module_06_rollingupdates4.svg)

## Service Definitions

This service balances across the Pods in a hypothetical nginx Deployment.

``` yaml
apiVersion: v1
kind: Service
metadata:
  name: nginx-service
spec:
  ports:
  - port: 8000
    # The container on each Pod to connect to.
    targetPort: 80
    protocol: TCP
  # Label selector to identify the set of pods to load-balance
  # traffic to.
  selector:
    app: nginx
```

This service can be created with:

``` console
$ kubectl create -f ./service.yaml
```

# Labels

Labels are key-value pairs that can be attached to each object in Kubernetes. They can be listed with the `labels:` section in the `metadata:` section of a Pod definition:

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
  labels:
    app: nginx
spec:
  containers:
  - name: nginx
    image: nginx
    ports:
    - containerPort: 80
```

Pods can then be queried based on their labels:

``` console
$ kubectl get pods -l app=nginx
```

# Health Checking

The Kubelet agent is in charge of health checking.

The simplest form is a process-level health check, where the Kubelet continuously asks the Docker daemon if the container process is still running, and restarts it if not. This health check is on by default for every container that runs in Kubernetes.

Kubernetes also supports application-level health checks, which can detect issues such as Deadlocks [^deadlocks]. There are three kinds of application health checks:

[^deadlocks]: Which would otherwise appear as healthy to process-level health checks, since the process is still running.

1. HTTP Health Checks: The Kubelet calls a web hook where a response code between 200 and 399 is considered success, and failure otherwise.
2. Container Exec: The Kubelet executes a command whose exit status determines the application's health: 0 is success, failure otherwise.
3. TCP Socket: The Kubelet attempts to open a socket to the container. If the connection is established, it's considered healthy, failure otherwise.

If the Kubelet discovers a failure, the container is restarted.

Health checks are configured in the `livenessProbe` section of the container configuration. It's also possible to specify an `initialDelaySeconds` setting to declare a grace period to allow for proper container initialization.

This is an example HTTP Health Check:

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-with-http-healthcheck
spec:
  containers:
  - name: nginx
    image: nginx
    livenessProbe:
      # HTTP health check.
      httpGet:
        path: /_status/healthz
        port: 80
      # Initialization grace period.
      initialDelaySeconds: 30
      # Timeout before considering failure.
      timeoutSeconds: 1
    ports:
    - containerPort: 80
```

Here is an example of a TCP Socket health check:

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: pod-with-tcp-socket-healthcheck
spec:
  containers:
  - name: redis
    image: redis
    livenessProbe:
      # TCP Socket health check.
      tcpSocket:
        port: 6379
      # Initialization grace period.
      initialDelaySeconds: 30
      # Timeout before considering failure.
      timeoutSeconds: 1
    ports:
    - containerPort: 6379
```

# ConfigMap

A ConfigMap decouples configuration artifacts from the image content in order to keep containerized applications portable. The data can be consumed in Pods or they may provide the configurations for system components such as controllers.

A ConfigMap can be created with the `create configmap` command from directories, files, or literal values:

``` console
$ kubectl create configmap :map-name :data-source
```

ConfigMaps can be created from literal key-value pairs with the `--from-literal` argument.

A ConfigMap can be created from a directory by passing a directory to the `--from-file` argument, in which case all of the files are combined. Otherwise a single file may be passed. The `--from-file` argument may be passed multiple times.

It's also possible to specify a different key to appear under the resulting `data:` section instead of the file name itself, by using the form `--from-file=:key=:path`.

``` console
$ ls config/dir/
game.properties
ui.properties

$ kubectl create configmap game-config --from-file=config/dir/

$ kubectl describe configmaps game-config
Name:           game-config
Namespace:      default
Labels:         <none>
Annotations:    <none>

Data
====
game.properties:        158 bytes
ui.properties:          83 bytes
```

The contents of the files are nested under the ConfigMap's `data:` section:

``` yaml
apiVersion: v1
data:
  game.properties: |
    enemies=aliens
    lives=3
    enemies.cheat=true
    enemies.cheat.level=noGoodRotten
    secret.code.passphrase=UUDDLRLRBABAS
    secret.code.allowed=true
    secret.code.lives=30
  ui.properties: |
    color.good=purple
    color.bad=yellow
    allow.textmode=true
    how.nice.to.look=fairlyNice
kind: ConfigMap
metadata:
  creationTimestamp: 2016-02-18T18:52:05Z
  name: game-config
  namespace: default
  resourceVersion: "516"
  selfLink: /api/v1/namespaces/default/configmaps/game-config-2
  uid: b4952dc3-d670-11e5-8cd0-68f728db1985
```

ConfigMaps may be referenced from Pod definitions with the `valueFrom.configMapKeyRef:` section. For example, given a ConfigMap named <span class="path">special-config</span> with a key `special.how` set to `very`, this Pod would use that key's value to set the `SPECIAL_LEVEL_KEY` environment variable:

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: dapi-test-pod
spec:
  containers:
    - name: test-container
      image: gcr.io/google_containers/busybox
      command: [ "/bin/sh", "-c", "env" ]
      env:
        # Define the environment variable
        - name: SPECIAL_LEVEL_KEY
          valueFrom:
            configMapKeyRef:
              name: special-config
              key: special.how
  restartPolicy: Never
```

It's also possible to use the `envFrom:` section to define all of a ConfigMap's data as Pod environment variables. It's also possible to expand such environment variables with the `$(VAR_NAME)` substitution syntax.

It's also possible to populate a Volume with data stored in a ConfigMap by referencing the ConfigMap with the `volumes.configMap:` section.

This can be further controlled by specifying the `volumes.configMap.items:` section to explicitly specify the path for specific ConfigMap items.

The following Pod definition results in files <span class="path">/etc/config/keys</span> and <span class="path">/etc/config/special.type</span>.

``` yaml
apiVersion: v1
kind: Pod
metadata:
  name: dapi-test-pod
spec:
  containers:
    - name: test-container
      image: gcr.io/google_containers/busybox
      command: [ "/bin/sh", "-c", "ls /etc/config/" ]
      volumeMounts:
      - name: config-volume
        mountPath: /etc/config
  volumes:
    - name: config-volume
      configMap:
        name: special-config
        items:
        - key: special.level
          path: keys
  restartPolicy: Never
```

The Kubelet periodically checks whether the mounted ConfigMap is fresh and updates it if it is not. This may take as much time as the Kubelet sync period + the TTL of the ConfigMaps cache in the Kubelet.

# Minikube

Minikube is a light-weight Kubernetes implementation that creates a local virtual machine and deploys a simple cluster containing a single Node [^docker_compose].

[^docker_compose]: This makes Kubernetes usable even in a local development environment, similar to what [Docker Compose] would achieve.

[Docker Compose]: /notes/docker#docker-compose

The `start` command will start a virtual machine with a Kubernetes cluster.

Instead of building a Docker image on the host and pushing it to a registry, the image can be built using the Docker installation on Minikube's VM, so that the images are automatically present. To accomplish this it's necessary to load the Minikube's Docker daemon environment:

``` console
$ eval $(minikube docker-env)

# Can be undone:
$ eval $(minikube docker-env -u)
```

# kubectl

The `kubectl`'s context determines which cluster it interacts with. Available contexts are shown in <span class="path">~/.kube/config</span>. The context can be set with the `config use-context :context` command.

The general structure of `kubectl` commands is:

``` console
$ kubectl :action :resource
```

The `--help` argument can be appended to most commands to get information about possible parameters.

The `version` shows the client and server version information.

The `cluster-info` command shows the master, dashboard, and other running applications.

The `get` command lists resources. For example, the `get nodes` command shows all Nodes that can be used to host applications.

The `describe` command shows detailed information about a resource.

The `logs` command prints the logs from a container in a Pod.

The `exec` command executes a command on a container in a Pod.

The `run` command creates a new Deployment. It requires a Deployment name and application image location, which should include the full repository URL for images hosted outside of the official Docker registry.

``` console
$ kubectl run kubernetes-bootcamp \
    --image=docker.io/jocatalin/kubernetes-bootcamp:v1 \
    --port=8080
```

This has the effect of searching for a suitable Node where an instance of the application can run, scheduling it to run on that Node, then configuring the cluster to reschedule the instance on a new Node when needed.

Deployments can be listed with the `get deployments` command.

Pods running inside of Kubernetes run in a private isolated network. By default they're visible from other Pods and Services within the same cluster, but not outside of that network.

The `proxy` command can be used to create a proxy through which it's possible to communicate with the cluster-wide private network.

`kubectl` interacts with the application through an API endpoint. The API server also automatically creates an API endpoint for each Pod based on its name.

The Pod name can be obtained with the following command:

``` console
export POD_NAME=$(kubectl get pods -o go-template --template '{{range .items}}{{.metadata.name}}{{"\n"}}{{end}}')

$ curl http://localhost:8001/api/v1/proxy/namespaces/default/pods/$POD_NAME/
```
