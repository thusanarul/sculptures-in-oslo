defmodule SculpturesInOslo.State.Descriptions do
  use Agent

  def start_link(initial_value) do
    Agent.start_link(fn -> initial_value end, name: __MODULE__)
  end

  def get_descriptions do
    Agent.get(__MODULE__, & &1)
  end

  def add_description(text) do
    Agent.update(__MODULE__, &[text | &1])
  end

  def reset do
    Agent.update(__MODULE__, fn _ -> [] end)
  end

  def get_empty_descriptions do
    __MODULE__.get_descriptions() |> Enum.filter(&(String.length(&1.text) == 0))
  end
end
