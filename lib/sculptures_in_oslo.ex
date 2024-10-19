defmodule SculpturesInOslo do
  @baseUrl "https://okk.kunstsamlingen.no"
  @moduledoc """
  Documentation for `SculpturesInOslo`.
  """

  @doc """
  Get links to visit from main page

  ## Examples

      iex> SculpturesInOslo.get_links_from_page(1)
      [{:title, ["a sculpture title"], :link, "/a-relative-link" }]

  """
  def get_links_from_page(pagenr) do
    req = Req.new(http_errors: :raise)

    body =
      Req.get!(req,
        url: main_page(pagenr)
      ).body

    {:ok, document} = Floki.parse_document(body)

    links_to_visit =
      document
      |> Floki.find("div.title a.detailLink")
      |> Enum.map(fn x ->
        {:title, Floki.attribute(x, "a", "title"), :link, Floki.attribute(x, "a", "href")}
      end)

    links_to_visit
  end

  defp main_page(pagenr) do
    "#{@baseUrl}/objects/images?filter=classification%3ASkulptur&page=#{pagenr}&sort=Relevance#filters"
  end
end
