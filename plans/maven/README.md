# Apache Maven 3

See [https://maven.apache.org/index.html](https://maven.apache.org/index.html) for more information.


To use this plan, you'll need to set the `JAVA_HOME` and pay attention to the location of `M2_HOME` during packaging.

```
export M2_HOME=/foo/bar/baz

# Ant requires JAVA_HOME to be set, and can be set via:
export JAVA_HOME=$(hab pkg path core/jdk8)
```


To use `mvn` in the Habitat dev shell:

```
hab pkg binlink core/maven mvn
```
