defmodule SculpturesInOslo.FetchWhere do
  use Task

  alias SculpturesInOslo.Prompt
  alias SculpturesInOslo.State.Descriptions
  alias SculpturesInOslo.State.Where

  def fetch_all do
    descriptions = Descriptions.get_descriptions()
    length_chunk = descriptions |> length |> Kernel./(4) |> ceil()
    chunked = descriptions |> Enum.chunk_every(length_chunk) |> Enum.with_index()

    start = System.monotonic_time(:millisecond)

    Task.async_stream(
      chunked,
      fn {chunk, index} ->
        # TODO: Ensure there are only as many chunks as servers running another way
        cond do
          index <= 3 ->
            server_port = 11434 + index
            IO.puts("Sending chunk to port: #{server_port}")
            get_where(chunk, server_port)

          index > 3 ->
            server_port = 11434 + 3
            IO.puts("Sending chunk to port: #{server_port}")
            get_where(chunk, server_port)
        end
      end,
      [{:timeout, :infinity}]
    )
    |> Stream.run()

    diff = System.monotonic_time(:millisecond) - start

    IO.puts("Took #{diff / 1000} seconds to fetch whereabouts:)")

    # get_where(descriptions)
    all = Where.get_where()

    {:ok, all |> length}
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
