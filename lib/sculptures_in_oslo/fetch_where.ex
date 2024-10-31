defmodule SculpturesInOslo.FetchWhere do
  use Task

  alias SculpturesInOslo.Prompt
  alias SculpturesInOslo.State.Descriptions
  alias SculpturesInOslo.State.Where

  def fetch_all do
    descriptions = Descriptions.get_descriptions()
    length_chunk = descriptions |> length |> Kernel./(4) |> floor()
    chunked = descriptions |> Enum.chunk_every(length_chunk)

    # Task.async_stream(chunked, fn chunk -> get_where(chunk) end) |> Task.await_many()

    get_where(descriptions)
    all = Where.get_where()

    {:ok, all}
  end

  def get_where([head | tail], server_port \\ 11434) do
    where = Prompt.get_wherabouts(head.text, server_port)

    Where.add_where(%{
      title: head.title,
      where: where,
      link: head.link
    })

    get_where(tail, server_port)
  end

  def get_where([], server_port) do
    IO.puts("Done with using #{server_port}")
    {:ok}
  end
end
