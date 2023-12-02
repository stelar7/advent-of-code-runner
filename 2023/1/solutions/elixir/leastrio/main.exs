defmodule Aoc do
  def parse(""), do: []
  def parse(<<c>> <> rest) when c in ?1..?9, do: [c - ?0 | parse(rest)]
  def parse(<<_>> <> rest), do: parse(rest)

  def parse_both(""), do: []
  def parse_both(<<c>> <> rest) when c in ?1..?9, do: [c - ?0 | parse_both(rest)]
  words = %{"one" => 1, "two" => 2, "three" => 3, "four" => 4, "five" => 5, "six" => 6, "seven" => 7, "eight" => 8, "nine" => 9}
  for {word, num} <- words do
    def parse_both(unquote(word) <> rest) do
      [unquote(num) | parse_both(rest)]
    end
  end
  def parse_both(<<_>> <> rest), do: parse_both(rest)
end

lines = IO.read(:all) |> String.trim() |> String.split("\n")
{p1, p2} = Enum.reduce(lines, {0, 0}, fn line, {t1, t2} ->
  nums1 = Aoc.parse(line)
  nums2 = Aoc.parse_both(line)
  {t1 + (10 * List.first(nums1)) + List.last(nums1), t2 + (10 * List.first(nums2)) + List.last(nums2)}
end)

IO.puts("#{p1}\n#{p2}")
