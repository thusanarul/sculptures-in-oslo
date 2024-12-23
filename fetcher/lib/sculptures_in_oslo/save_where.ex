defmodule SculpturesInOslo.SaveWhere do
  @path ~S"tmp/data.csv"
  alias SculpturesInOslo.State.Where

  def save_to_file do
    where_data = Where.get_where()

    data =
      Enum.reduce(where_data, ["title,where,link"], fn %{
                                                         :title => title,
                                                         :where => where,
                                                         :link => link
                                                       },
                                                       acc ->
        [["\'#{title |> hd}\',\'#{where}\',\'#{link}\'"] | acc]
        # [["#{chunk.title},#{chunk.where},#{chunk.link}"] | acc]
      end)
      |> Enum.reverse()
      |> Enum.join("\n")

    IO.puts("Writing to #{@path}")
    File.write!(@path, data)
  end
end
