defmodule SculpturesInOslo.Prompt do
  use Task

  @prompt_base "Tell me where the statue is currently at based only on the provided text. Give me the answer in one line and only where it is located. The text: "
  @ollama_ports [11434, 11435, 11436, 11437]
  @ollama_running [false, false, false, false]

  def start_link() do
    Task.start_link(__MODULE__, :init, [])
  end

  def init do
    shell_cmd =
      Enum.map(@ollama_ports, fn port ->
        "OLLAMA_HOST=127.0.0.1:#{port} ollama serve"
      end)
      |> Enum.join(" & ")
  end
end
