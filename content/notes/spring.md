+++
title = "Spring"
date = 2022-03-27

[note]
kind = "technology"
+++

[Spring](https://spring.io/) is an application framework and inversion of control container for Java.

These notes focus on [Spring Boot](https://spring.io/projects/spring-boot), which is a distribution of Spring and related packages geared toward web application development.

<nav id="toc"></nav>

# Configuration

Configuration applies based on a [predefined precedence](https://docs.spring.io/spring-boot/docs/current/reference/html/features.html#features.external-config).

It appears that an `application.yml` in the `test/resources/` completely shadows one in `main/resources/`. As a result, you can't depend on necessary configuration to exist in the `main` and only overriding certain attributes in the `test` one.

Due to the auto-configuration mechanism described below, this could result in classes/functionality you expect to be taking effect, to not take effect during your tests.

A `@Configuration` class can be loaded based on the active profiles by using the `@Profile("some_profile")` annotation. This annotation supports an expressive grammar allowing for rules such as `"!a & !b"` to mean that it should only be loaded if neither of profile `a` or `b` are loaded.

### Overriding Configurations for Tests

Sometimes it can be useful to override certain beans in a test environment. This can be accomplished by enabling overrides in tests:

```ini
spring.main.allow-bean-definition-overriding=true
```

Then mark a test-only configuration with `@TestConfiguration` instead of `@Configuration`.

Finally, explicitly import this configuration in the test that should apply it by annotating it with the `@Import` annotation:

```java
@Import(MyTestConfiguration.class)
class SomeTest {
  …
}
```

Instead of explicitly importing the configuration, if a configuration is very specific to a given test and likely not to be reused, it can simply be defined as a static inner class, in which case the enclosing test will automatically discover and load it.

```java
@SpringBootTest
public class SomeTest {
  @Autowired SomeClass someClass;

  @Test
  void someTest() { … }

  @TestConfiguration
  public static class Configuration {
    @Bean
    public SomeClass getSomeClass() {
      return new SomeClass();
    }
  }
}
```

## Auto-Configuration

Spring Boot auto-configures different application components based on dependencies present in the classpath (e.g. a database pool if a database dependency is detected). It appears it also does this based on the presence of properties, and many other reasons.

In general, there are [several annotations](https://docs.spring.io/spring-boot/docs/current/reference/html/features.html#features.testing.spring-boot-applications.additional-autoconfiguration-and-slicing) applied to certain classes which determine whether or not a class/configuration gets loaded. For example, the following appears to depend on classes `GraphQL` and `GraphQlSource` being in the classpath, the `GraphQlSource` bean not already existing, and the `GraphQlProperties` properties being defined, which it appears means at least the "root" (i.e. `spring.graphql`) such that the object gets created.

```java
@Configuration(proxyBeanMethods = false)
@ConditionalOnClass({GraphQL.class, GraphQlSource.class})
@ConditionalOnMissingBean(GraphQlSource.class)
@EnableConfigurationProperties(GraphQlProperties.class)
public class GraphQlAutoConfiguration {
  …
```

These conditions can form a dependency graph. For example, this next class itself appears to depend on the above class having been loaded.

```java
@ConditionalOnWebApplication(type = ConditionalOnWebApplication.Type.SERVLET)
@ConditionalOnClass({GraphQL.class, GraphQlHttpHandler.class})
@ConditionalOnBean(GraphQlSource.class)
@AutoConfigureAfter(GraphQlServiceAutoConfiguration.class)
public class GraphQlWebMvcAutoConfiguration {
  …
```

This can be [opted-out of](https://docs.spring.io/spring-boot/docs/current/reference/html/using.html#using.auto-configuration.disabling-specific) on a per-class basis with the annotation:

```java
@SpringBootApplication(exclude = { DataSourceAutoConfiguration.class })
public class MyApplication {
  …
}
```

# Logging

Startup logging can be disabled with `spring.main.log-startup-info=false`.

You can produce more in-depth logging (full conditions report) by enabling the `debug` property with e.g. `--debug` or `-Ddebug=true`.

# Debugging

## When a Configuration does not Apply

When a configuration doesn't appear to apply, set the `org.springframework` log-level to `DEBUG`:

```xml
<logger name="org.springframework" level="DEBUG" />
```

This will emit messages of the form:

```
GraphQlWebMvcAutoConfiguration:
  Did not match:
      - @ConditionalOnBean (types: org.springframework.graphql.ExecutionGraphQlService; SearchStrategy: all) did not find any beans of type org.springframework.graphql.ExecutionGraphQlService (OnBeanCondition)
  Matched:
      - @ConditionalOnClass found required classes 'graphql.GraphQL', 'org.springframework.graphql.web.webmvc.GraphQlHttpHandler' (OnClassCondition)
      - found 'session' scope (OnWebApplicationCondition)
```
