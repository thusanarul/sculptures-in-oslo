defmodule SculpturesInOslo.Application do
  use Application

  def start(_type, _args) do
    children = [
      SculpturesInOslo.State.LinksToVisit,
      SculpturesInOslo.State.Descriptions,
      SculpturesInOslo.State.Where,
      SculpturesInOslo.Prompt,
      SculpturesInOslo.FetchLinks
    ]

    opts = [strategy: :one_for_one, name: SculpturesInOslo.Supervisor]
    Supervisor.start_link(children, opts)
  end
end
