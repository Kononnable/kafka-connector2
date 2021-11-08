# kafka-connector2

Apache Kafka client library written in Rust as rewrite of Java version.

Warning: This project is no close to being stable or feature-rich. It might never reach that state - lack of time and complexity of the subject.

## Testing
Testing is done using original kafka test suites. It guarantees functional compatibility between Java and Rust version and test many corner cases which where discovered over the years.

To test use one of the following commands inside kafka directory:
```shell
./gradlew test
./gradlew unittest
./gradlew integrationtest
```
Note: Kafka integration tests consume a lot of resources. They spawn multiple kafka server instances underneath which can overcommit CPU and consume a lot of RAM(20GB+). It might be wise to limit number of tests running at the same time(e.g. `--no-parallel --max-workers=1`) to use less resources over much longer testing time.

## Licensing
Rust crates are distributed under the terms of both the MIT license and the Apache License (Version 2.0). 

Kafka directory contains snapshot of source code of Apache Kafka(https://kafka.apache.org/) with some integration changes. Apache Kafka is licensed under Apache License (Version 2.0).
Apache Kafka code is used only for testing purposes(dev-dependency).