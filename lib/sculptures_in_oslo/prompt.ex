defmodule SculpturesInOslo.Prompt do
  use Task
  alias SculpturesInOslo.Prompt.OllamaServers

  @prompt_base "Tell me where the statue is currently at based only on the provided text. Answer me with the current location only, preferably one word. If you can not find the current location, answer 'N/A'. The text: "
  @model "llama3.2"
  @ollama_server_ports [11434, 11435, 11436, 11437]
  # @ollama_server_ports [11434]

  def start_link(arg \\ []) do
    OllamaServers.start_link()
    Task.start_link(__MODULE__, :init, [arg])
  end

  def init(_arg) do
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
    index = @ollama_server_ports |> Enum.find_index(fn p -> p == server_port end)
    OllamaServers.set_port_running(index, true)

    prompt = "#{@prompt_base}#{description}"

    env = %{"OLLAMA_HOST" => "127.0.0.1:#{server_port}"}
    {:ok, response} = Rambo.run("ollama", ["run", @model, prompt], env: env)

    OllamaServers.set_port_running(index, false)
    response.out |> String.trim()
  end
end

defmodule SculpturesInOslo.Prompt.OllamaServers do
  use Agent

  def start_link() do
    running = [false, false, false, false]
    port_pids = [nil, nil, nil, nil]

    state = %{running: running, port_pids: port_pids}

    Agent.start_link(fn -> state end, name: __MODULE__)
  end

  def update_port_pid(index, port_pid) do
    Agent.update(__MODULE__, fn state ->
      port_pids = state.port_pids |> List.replace_at(index, port_pid)
      %{state | port_pids: port_pids}
    end)
  end

  def set_port_running(index, running) do
    Agent.update(__MODULE__, fn state ->
      running = state.running |> List.replace_at(index, running)
      %{state | running: running}
    end)
  end

  def get_state() do
    Agent.get(__MODULE__, & &1)
  end
end
