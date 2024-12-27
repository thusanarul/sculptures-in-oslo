#!/usr/bin/env elixir

Mix.install([
  :req,
  {:jason, "~> 1.0"},
  {:csv, "~> 3.2"}
])

defmodule GPS.Fetch do
  @g_api_key System.get_env("GOOGLE_MAPS_API_KEY")
  @base_url "https://maps.googleapis.com/maps/api/place/findplacefromtext/json"

  defp pos_as_map(pos) when is_map(pos) do
    addr = pos |> Map.get("formatted_address", nil)
    lat = pos |> get_in(["geometry", "location", "lat"])
    lon = pos |> get_in(["geometry", "location", "lng"])
    %{lat: lat, lon: lon, address: addr}
  end

  defp pos_as_map(_pos) do
    %{lat: nil, lon: nil, address: nil}
  end

  defp get_request_url(where) do
    Req.new(
      url: @base_url,
      params: [
        key: @g_api_key,
        input: where,
        inputtype: "textquery",
        fields: "name,formatted_address,geometry/location",
        locationbias: "circle:25000@59.9139,10.7522"
      ]
    )
  end

  def fetch_pos(where) do
    body =
      get_request_url(where)
      |> Req.get!()
      |> then(& &1.body)

    pos = body["candidates"] |> Enum.at(0) |> pos_as_map()
    pos
  end

  def add_pos(row) do
    where = row |> Map.get("where") |> String.trim()
    title = row |> Map.get("title") |> String.trim()
    link = row |> Map.get("link") |> String.trim()

    pos = fetch_pos(where)
    %{title: title, where: where, address: pos.address, lat: pos.lat, lon: pos.lon, link: link}
  end
end

sculptures =
  "../.thusanarul/skulpturer.csv"
  |> Path.expand(__DIR__)
  |> File.stream!()
  |> CSV.Decoding.Decoder.decode(
    separator: ?;,
    headers: true
  )
  |> Stream.map(fn {:ok, map} -> Map.drop(map, [""]) end)
  |> Stream.map(&GPS.Fetch.add_pos(&1))
  |> CSV.Encoding.Encoder.encode(headers: true)
  |> Stream.into(
    "../.thusanarul/skulpturer_m_pos.csv"
    |> Path.expand(__DIR__)
    |> File.stream!([:write, :utf8, :delayed_write])
  )
  |> Enum.to_list()

IO.puts("Wrote #{sculptures |> Enum.count()} lines to file!")
