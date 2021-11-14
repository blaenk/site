+++
title = "Minio"
date = 2021-11-08

[work]
kind = "contribution"
+++

Minio is a Kubernetes Native Object Storage system. One of its features is that it can provide an S3-compatible interface on top of HDFS.

After a company merger, Minio was used to establish an S3-interface bridge so that engineers on integrating teams could access data on legacy HDFS. However, its use was hamstrung by excruciatingly slow directory listings for reasons which eluded everyone for months. For very large directories of +1,500 files, it was prohibitively slow. This affected both CLI use and Spark use.

The Minio installation was under the purview of the infrastructure engineering team (DevOps), and I did not have to use it myself, nor did my team, so I had not been aware of—nor had to endure—the issue. My manager regularly met with the director of infrastructure engineering to sync up, and after months of enduring this issue and being at a loss as to its cause, my manager relayed to me that this was a problem and that I may be interested in taking a look, since I had always demonstrated a passion for getting to the bottom of issues.

For starters, I trusted that the infrastructure engineering team had vetted their installation and configuration (but I kept an open mind on this with respect to revisiting it later), so I proceeded to jump straight into the Minio source code. I set up a debugging environment, ensuring I could debug it at all and that I could build the server and client with modifications.

I profiled the code and nothing really stood out as being a bottleneck. I inspected the code and nothing jumped out at me as obviously incorrect. I developed the hunch that maybe Minio was performing a separate HDFS network call (equivalent to `stat()`) per directory entry. After some investigation, this proved to be the case, and swapping it out with a single pipelined network call sped up directory listings of large directories with +1,500 files by 200x, to the relief of many who had to regularly interact with Minio.

See the PRs [here](https://github.com/minio/minio/issues?q=author%3Ablaenk+is%3Amerged).
