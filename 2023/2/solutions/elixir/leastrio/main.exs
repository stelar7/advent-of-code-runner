defmodule Aoc do
  @colors %{
    "red" => 12,
    "green" => 13,
    "blue" => 14
  }

  def update_val(existing, new) do
    if existing > new do
      existing
    else
      new
    end
  end

  def run(input) do
    meow = input
      |> String.trim()
      |> String.split("\n")
      |> Enum.flat_map(fn line ->
        [head, data] = String.split(line, ":")
        ["Game", id] = String.split(head, " ")
        {valid, power} = String.trim(data)
          |> String.replace(";", ",")
          |> String.split(",")
          |> Enum.reduce({true, %{"red" => 1, "green" => 1, "blue" => 1}}, fn roll, {valid, color_map} ->
            [num, color] = String.trim(roll) |> String.split(" ")
            valid = if valid do @colors[color] >= String.to_integer(num) else false end
            new_map = color_map
              |> Map.update(color, color_map[color], fn v -> update_val(v, String.to_integer(num)) end)
            {valid, new_map}
          end)
        [{String.to_integer(id), valid, power}]
      end)
    valid_sum = Enum.reduce(meow, 0, fn {id, valid, _}, total -> if valid do total + id else total end end)
    sum_power = Enum.reduce(meow, 0, fn {_, _, %{"red" => r, "green" => g, "blue" => b}}, total -> total + (r * g * b) end)
    IO.puts("#{valid_sum}\n#{sum_power}")
  end
end

Aoc.run(IO.read(:all))
