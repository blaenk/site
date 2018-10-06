+++
title = "Ruby on Rails"
date = 2017-01-30

[note]
kind = "technology"
+++

<nav id="toc"></nav>

# Bundler

Bundler performs dependency resolution on the entire list of dependencies.

The `install` command should be invoked whenever dependencies are modified.

## Gemfile

The `:require` option can be used to specify the gem to use when it differs from the name.

``` ruby
gem 'webmock', require: 'webmock/rspec'
```

The `:path` option can be used to use a local gem.

``` ruby
gem 'nokogiri', path: '~/code/nokogiri'
```

A git source repository can be used with the `:git` option to load a gem as long as it has a <span class="path">.gemspec</span> file in its root. This can be shortened when the repository is on GitHub by using the `:github` option. It's also possible to use the `:ref`, `:branch`, or `:tag` options to anchor it to a particular revision.

``` ruby
gem 'carrierwave', git: 'git@github.com:carrierwaveuploader/carrierwave.git'
gem 'carrierwave', github: 'carrierwaveuploader/carrierwave'
```

## Gem Locking

Running the `install` or `update` commands causes Bundler to resolve the dependency tree and save it in the <span class="path">Gemfile.lock</span> file. This "locks" the versions of the gems being used.

The `package` command can be used to package up all of the dependencies into <span class="path">vendor/cache</span>. Vendored dependencies can then be installed using `install --local`.

Dependency scripts can be run within the application's RubyGem's environment using the `exec` command.

Rails 4 and greater automatically creates _binstubs_---scripts that run in the bundle's context---for Rails executables such as bundle, rake, and rails. It's also possible to create binstubs for a dependency using the `binstubs` command.

# Configuration Files

Changes to configuration files require a server restart to take effect, since they're only run at application startup.

The <span class="path">config/boot.rb</span> file is for setting up Bundler and load paths.

The <span class="path">config/application.rb</span> file is for loading rails gems, as well as those for the current Rails environment `Rails.env`, and for configuration the application. It's a good place to store application settings. It can also be used to specifically pick "railties" to load, i.e. Rails gems. This file also loads <span class="path">config/boot.rb</span>, and defines a module for the application, allowing for multiple instances of the application within the same process.

The <span class="path">config/application.rb</span> file can also be used to configure generators, such as specifying what template engine to use.

The <span class="path">config/environment.rb</span> file is used to load initializers. This should contain <span class="path">config/application.rb</span> at the top.

## Initializers

Initializers are a Rails convention for breaking up pieces of configuration into self-contained files. Initializers live in <span class="path">config/initializers/</span>.

The <span class="path">backtrace\_silencers.rb</span> initializer allows configuring the shortening of backtraces.

The <span class="path">filter\_parameter\_logging.rb</span> initializer allows filtering out sensitive information from logs.

The <span class="path">inflections.rb</span> file can be used to configure singular↔plural transformations.

The <span class="path">session\_store.rb</span> initializer can be used to configure the encrypted session store's key, though the secret key itself is stored in <span class="path">config/secrets.yml</span> and the `rake secret` command can be used to generate one automatically.

## Load Paths

Rails automatically looks in certain directories, such as any within <span class="path">app/</span>. The load path can modified via `config.autoload_paths`.

The default log level of `:debug` can be changed via `config.log_level`.

## Console

The console environment can be configured by passing a block to the `console` method in <span class="path">config/application.rb</span>.

``` ruby
console do
  require 'pry'
  config.console = Pry
end
```

## Environments

The <span class="path">config/environments/</span> directory stores configuration files specific to particular environments, such as <span class="path">development.rb</span>.

## Development Mode

Rails 4.1 ships with an application preloader named Spring which monitors <span class="path">config/</span> and <span class="path">initializers/</span>, as well as any changed gem dependencies for changes and then automatically restarts the application when any are detected.

Development mode also enables automatic class reloading. The `config.cache_classes` setting. When `true`, Rails uses `require` to load files instead of `load`. The difference is that `require` caches the file so that subsequent `require`s are ignored, unlike `load`.

Code is not eagerly loaded when in development mode in order to increase startup times. This can be changed via `config.eager_load`.

## Rails Class Loader

Rails registers a callback for missing constants so that when a previously unknown class is encountered, Rails conventions are used to attempt to locate and load the file that would contain that class. In particular, the conventions are:

* If the class/module is not nested, convert the name to lower-snake-case:

    `SourceCode` becomes `source_code`

* If the class/module is nested, each level of nesting represents a sub-directory and the above conversion takes places as well.

    `SourceCode::PullRequest` becomes `source_code/pull_request`

## Database Configuration

The <span class="path">database.yml</span> file is used to configure the database. It's advised not to store it in version control because it may contain sensitive information as well as different development and test setups among team members.

Rails 4.1 added support for configuring Active Record with the `DATABASE_URL` environment variable.

## Application Secrets

The <span class="path">secrets.yml</span> file is for storing application secrets. It's advised not to store any sensitive production information in it if checked into version control. Instead delegate to environment variables:

``` yml
production:
  secret_key_base: ENV['SECRET_KEY_BASE']
```

Secrets are accessible through the `Rails.application.secrets` hash.

## Logging

Most Rails contexts expose a `logger` attribute that is a reference to the logger. This is also accessible via `Rails.logger`.

It's possible to list request object methods in `config.log_tags` to have that information show up in the logs. For example, setting `[:subdomain]` will prepend each request's subdomain at the beginning of its log message.

# Routing

Routes map URL paths to controller and action pairs, and they're defined in <span class="path">config/routes.rb</span>. For example, to route GET <span class="path">products/:id</span> to the `Products` controller's `show` action, one can do:

``` ruby
get 'products/:id', to: 'products#show'
get 'products/:id' => 'products#show'
```

The `match` method is more general, and its `via:` argument can take one or more HTTP methods to restrict the route to.

``` ruby
match 'products/:id' => 'products#show', via: :get
```

_Segment keys_ are URL parameters denoted by a colon `:` prefix, similar to Ruby symbols, for example <span class="path">products/:id</span>.

URLs to defined routes can be generated automatically using the `link_to` method, which takes as parameters the text to show for the link, the controller, action, and its URL parameters.

``` ruby
link_to "Products",
  controller: "products",
  action: "show",
  id: 1
```

There are also more ergonomic _named routes_ that can be used to generate URLs from defined routes.

_Optional segment keys_ can be defined by wrapping the segments in parentheses. For example, to require a `:controller` URL parameter but also have an _optional_ `:action` parameter:

``` ruby
match ':controller(/:action)', via: :any
```

Redirects can be specified in a route. It can take an optional HTTP `:status` code or even a block that is passed the request parameters.

``` ruby
get '/foo', to: redirect('/bar')
get '/with_status', to: redirect('/destination', status: 302)

get '/thing', to: redirect do |request_params|
  "/api/#{request_params[:api_version]}"
end
```

More generally, the `:to` parameter takes a _Rack Endpoint_, which the aforementioned `redirect` evaluates to. A Rack endpoint can be a callable that takes an environment hash and returns a 3-length array consisting of the HTTP status code, response headers, and response body.

In fact, the `"controller#action"` syntax relies on the `action` method on the `Controller` to return a Rack endpoint that actually invokes the specified action.

An arbitrary Rack-based application can be dispatched to by using the `mount` method, using the `:at` option to specify the route that will map to it.

``` ruby
class SinatraApp < Sinatra::Base
  get '/' { 'Test' }
end

Rails.application.routes.draw do
  mount SinatraApp, at: '/hello'
end
```

It's possible to respond to different formats by using the `respond_to` method.

``` ruby
def show
  @product = Product.find(params[:id])

  respond_to do |format|
    format.html
    format.json { render json: @product.to_json }
    format.any do
      # Any other format not previously handled.
    end
  end
end
```
