defmodule SculpturesInOslo.Prompt do
  use Task

  @prompt_base "Tell me where the statue is currently at based only on the provided text. Answer me with the current location only, preferably one word. If you can not find the current location, answer 'N/A'. The text: "
  @model "llama3.2"
  @ollama_server_ports [11434, 11435, 11436, 11437]
  # @ollama_server_ports [11434]

  def start_link(arg \\ []) do
    Task.start_link(__MODULE__, :init, [arg])
  end

  def init(_arg) do
    IO.puts("Opening ollama servers")

    Task.async_stream(@ollama_server_ports, fn port -> open(port) end, [{:timeout, :infinity}])
    |> Stream.run()

    IO.puts("Opened ollama servers...")

    {:ok, []}
  end

  def open(server_port \\ 11434) do
    IO.puts("Starting ollama server: #{server_port}")
    env = %{"OLLAMA_HOST" => "127.0.0.1:#{server_port}"}
    Rambo.run("ollama", ["serve"], env: env, log: false)

    {:ok, "started server: #{server_port}"}
  end

  def get_wherabouts(description, server_port \\ 11434) do
    prompt = "#{@prompt_base}#{description}"

    env = %{"OLLAMA_HOST" => "127.0.0.1:#{server_port}"}
    {:ok, response} = Rambo.run("ollama", ["run", @model, prompt], env: env)

    response.out |> String.trim()
  end
end
