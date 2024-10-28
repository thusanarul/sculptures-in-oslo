defmodule SculpturesInOslo.Prompt do
  use Task
  alias SculpturesInOslo.Prompt.OllamaServers

  @prompt_base "Tell me where the statue is currently at based only on the provided text. Give me the answer in one line and only where it is located. The text: "
  @model "llama3.2"
  @ollama_server_ports [11434, 11435, 11436, 11437]
  # @ollama_server_ports [11434]
  @wrapper_executable "./wrapper.sh"

  def start_link do
    OllamaServers.start_link()
    Task.start_link(__MODULE__, :init, [])
  end

  def init do
    Enum.map(@ollama_server_ports, fn port ->
      spawn(fn -> open(port) end)
    end)

    {:ok, []}
  end

  def open(server_port \\ 11434) do
    port =
      Port.open({:spawn_executable, @wrapper_executable}, [
        :binary,
        {:args, ["ollama", "serve"]},
        {:env, [{~c"OLLAMA_HOST", ~c"127.0.0.1:#{server_port}"}]}
        # {:env, [{"OLLAMA_HOST", "127.0.0.1:#{server_port}"}]}
      ])

    receive do
      {_, {:data, msg}} -> IO.puts(msg)
    end

    index = @ollama_server_ports |> Enum.find_index(fn p -> p == server_port end)
    OllamaServers.update_port_pid(index, port)

    IO.puts("Started server: #{server_port}")
    port
  end

  def get_wherabouts(description) do
    prompt = "#{@prompt_base}#{description}"

    response = Rambo.run("ollama", ["run", @model, prompt])
    response
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

  def get_state() do
    Agent.get(__MODULE__, & &1)
  end
end
