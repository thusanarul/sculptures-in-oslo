#!/usr/bin/env elixir

Mix.install([
  :req,
  {:jason, "~> 1.0"},
  {:csv, "~> 3.2"}
])

g_api_key = System.get_env("GOOGLE_MAPS_API_KEY")

sculptures =
  "../.thusanarul/skulpturer.csv"
  |> Path.expand(__DIR__)
  |> File.stream!()
  |> CSV.Decoding.Decoder.decode(
    separator: ?;,
    headers: true
  )
  |> Stream.map(fn {:ok, map} -> Map.drop(map, [""]) end)
  |> Enum.take(5)

IO.inspect(sculptures)
