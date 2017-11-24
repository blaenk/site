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

