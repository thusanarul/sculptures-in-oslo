defmodule SculpturesInOslo.FetchAndParse do
  use GenServer
  @baseUrl "https://okk.kunstsamlingen.no"
  @moduledoc """
  Documentation for `SculpturesInOslo`.
  """

  def start_link(opts) do
    GenServer.start_link(__MODULE__, :ok, opts)
  end

  def init(:ok) do
    first_page = get_links_from_page(1)

    IO.inspect(first_page)

    {:ok, [first_page]}
  end

  @doc """
  Get description of sculpture from page
  """

  def get_description_from_page(relative_link) do
    url = "#{@baseUrl}#{relative_link}"

    # TODO: figure out if i need multiple of this
    req = Req.new(http_errors: :raise)

    body =
      Req.get!(req,
        url: url
      ).body

    {:ok, document} = Floki.parse_document(body)

    document |> Floki.find("div.detailed-text span") |> Floki.text()
  end

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
