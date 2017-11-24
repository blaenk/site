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

# Nodes

A Node is a worker machine in Kubernetes. Multiple Pods can run on a single Node.

A Node can have multiple Pods. The Kubernetes master automatically handles scheduling Pods across the Nodes in the cluster based on the available resources on each Node. Each Node runs at least:

* Kubelet: a process responsible for communicating between the master and the Nodes in the cluster. It manages the Pods and the containers running on the machine.
* Container runtime: Such as Docker, responsible for pulling the container image from a registry, then unpacking and running it.

![node](https://d33wubrfki0l68.cloudfront.net/5cb72d407cbe2755e581b6de757e0d81760d5b86/a9df9/docs/tutorials/kubernetes-basics/public/images/module_03_nodes.svg)

# Services

A Service is an abstraction layer which defines a logical set of Pods and enables external traffic exposure, load balancing, and Service discovery for those Pods.

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

# Minikube

Minikube is a light-weight Kubernetes implementation that creates a local virtual machine and deploys a simple cluster containing a single Node [^docker_compose].

[^docker_compose]: This makes Kubernetes usable even in a local development environment, similar to what [Docker Compose] would achieve.

[Docker Compose]: /notes/docker#docker-compose

The `start` command will start a virtual machine with a Kubernetes cluster.

# kubectl

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
