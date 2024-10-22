defmodule SculpturesInOslo.LinksToVisit do
  use Agent

  def start_link(initial_value) do
    Agent.start_link(fn -> initial_value end, name: __MODULE__)
  end

  def get_links do
    Agent.get(__MODULE__, & &1)
  end

  def add_links(links) do
    Agent.update(__MODULE__, &(links ++ &1))
  end
end
