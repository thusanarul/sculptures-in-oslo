defmodule SculpturesInOslo.MixProject do
  use Mix.Project

  def project do
    [
      app: :sculptures_in_oslo,
      version: "0.1.0",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {SculpturesInOslo.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:req, "~> 0.5.0"},
      {:floki, "~> 0.36.0"},
      {:rambo, "~> 0.3"}
    ]
  end
end
