defmodule SculpturesInOslo.FetchDescriptions do
  alias SculpturesInOslo.LinksToVisit
  use Task
  @baseUrl "https://okk.kunstsamlingen.no"

  def start_link(arg) do
    Task.start_link(__MODULE__, :run, [arg])
  end

  def run do
    IO.puts("Starting descriptions fetcher:)")
    links = LinksToVisit.get_links()

    descs =
      Task.async_stream(links, fn link -> get_description_from_page(link.link) end)
      |> Enum.to_list()

    IO.inspect(descs)
  end

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
end
