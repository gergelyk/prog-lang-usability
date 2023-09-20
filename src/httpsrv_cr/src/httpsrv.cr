require "kemal"

get "/" do
  "Hello World!"
end

get "/bug" do
  raise "Intentional error"
end

get "/:name" do |env|
  name = env.params.url["name"]
  "Hello #{name}!"
end

Kemal.run
