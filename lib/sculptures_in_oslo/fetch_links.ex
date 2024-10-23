defmodule SculpturesInOslo.FetchLinks do
  use Task
  @base_url "https://okk.kunstsamlingen.no"
  @moduledoc """
  Documentation for `SculpturesInOslo`.
  """
  alias SculpturesInOslo.FetchDescriptions
  alias SculpturesInOslo.State.LinksToVisit

  def start_link(arg) do
    Task.start_link(__MODULE__, :run, [arg])
  end

  def run(_arg) do
    _links =
      Task.async_stream(1..26, fn pagenr -> get_links_from_page(pagenr) end)
      |> Stream.map(fn {:ok, links} -> LinksToVisit.add_links(links) end)
      |> Enum.to_list()

    IO.puts("Done fetching links!")

    spawn(&FetchDescriptions.run/0)
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
        %{
          title: Floki.attribute(x, "a", "title"),
          url: Floki.attribute(x, "a", "href") |> create_link
        }
      end)

    links_to_visit
  end

  defp create_link(relative_link) do
    "#{@base_url}#{relative_link}"
  end

  defp main_page(pagenr) do
    "#{@base_url}/objects/images?filter=classification%3ASkulptur&page=#{pagenr}&sort=Relevance#filters"
  end
end
