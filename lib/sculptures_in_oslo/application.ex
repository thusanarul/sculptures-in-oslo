defmodule SculpturesInOslo.Application do
  use Application

  def start(_type, _args) do
    children = [SculpturesInOslo.LinksToVisit]

    opts = [strategy: :one_for_one, name: SculpturesInOslo.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
