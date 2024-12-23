defmodule SculpturesInOslo.State.Where do
  use Agent

  def start_link(initial_value) do
    Agent.start_link(fn -> initial_value end, name: __MODULE__)
  end

  def get_where do
    Agent.get(__MODULE__, & &1)
  end

  def add_where(obj) do
    Agent.update(__MODULE__, &[obj | &1])
  end

  def reset do
    Agent.update(__MODULE__, fn _ -> [] end)
  end

  def get_empty_where do
    __MODULE__.get_where() |> Enum.filter(&(String.length(&1.text) == 0))
  end
end
