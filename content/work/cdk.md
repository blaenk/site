+++
title = "CDK"
date = 2021-11-08

[work]
kind = "contribution"
+++

AWS Cloud Development Kit (CDK) is an Infrastructure-as-Code (IaC) system developed by Amazon Web Services that builds on top of CloudFormation.

See the PRs [here](https://github.com/aws/aws-cdk/pulls?q=author%3Ablaenk+is%3Apr).

### Docker Ignore Features

CDK had a built-in support for processing `.dockerignore` files for its Docker Image asset handling, but instead of processing it according to `.dockerignore`'s own rules, it processed them using the [minimatch](https://github.com/isaacs/minimatch) library which aims to provide [`fnmatch(3)`](https://www.man7.org/linux/man-pages/man3/fnmatch.3.html) pattern matching behavior. This led to certain behaviors commonly found in `.dockerignore` files were not supported, such as writing it as a white-list. More confusingly, no error would be emitted, rather the rules would be processed in an unexpected way that would confuse the user.

To resolve this, I implemented support for _native_ handling of `.dockerignore` according to Docker's rules _and_ also implemented support for native handling of `.gitignore` which is much more common.

See the [PR](https://github.com/aws/aws-cdk/pull/10922).

### Allow Configuring WorkDir

AWS Lambda resources defined with CDK had no way of overriding the `WorkDir` CloudFormation property which allows overriding the working directory for a specified Docker image. I went ahead and exposed this property.

See the [PR](https://github.com/aws/aws-cdk/pull/16111).
