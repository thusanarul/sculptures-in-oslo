defmodule SculpturesInOslo do
  @moduledoc """
  Documentation for `SculpturesInOslo`.
  """

  @doc """
  Hello world.

  ## Examples

      iex> SculpturesInOslo.hello()
      :world

  """
  def hello do
    req = Req.new(http_errors: :raise) |> ReqEasyHTML.attach()

    body =
      Req.get!(req,
        url:
          "https://okk.kunstsamlingen.no/objects/images?filter=classification%3ASkulptur&sort=Relevance#filters"
      ).body

    IO.puts(body)
    :world
  end
end
