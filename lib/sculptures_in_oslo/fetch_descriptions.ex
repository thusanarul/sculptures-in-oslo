defmodule SculpturesInOslo.FetchDescriptions do
  alias SculpturesInOslo.State.Descriptions
  alias SculpturesInOslo.State.LinksToVisit
  use Task

  def start_link(arg) do
    Task.start_link(__MODULE__, :run, [arg])
  end

  def run do
    IO.puts("Starting descriptions fetcher:)")
    links = LinksToVisit.get_links()

    descs =
      Task.async_stream(links, fn link ->
        %{title: link.title, text: get_description_from_page(link.url), link: link.url}
      end)
      |> Stream.map(fn {:ok, descs} -> Descriptions.add_description(descs) end)
      |> Enum.to_list()

    IO.inspect(descs)
  end

  def get_description_from_page(url) do
    # TODO: figure out if i need multiple of this
    req = Req.new(http_errors: :raise)

    body =
      Req.get!(req,
        url: url
      ).body

    {:ok, document} = Floki.parse_document(body)

    node =
      document
      |> Floki.find("div.detailed-text, div.expanded-full-text")

    node |> Floki.text()
  end
end
