defmodule Aoc do
  defp parse(""), do: []
  defp parse(<<c>> <> rest) when c in ?1..?9, do: [c - ?0 | parse(rest)]
  defp parse(<<_>> <> rest), do: parse(rest)

  defp parse_both(""), do: []
  defp parse_both(<<c>> <> rest) when c in ?1..?9, do: [c - ?0 | parse_both(rest)]
  words = %{"one" => 1, "two" => 2, "three" => 3, "four" => 4, "five" => 5, "six" => 6, "seven" => 7, "eight" => 8, "nine" => 9}
  for {word, num} <- words do
    defp parse_both(unquote(word) <> _ = full_string) do
      <<_>> <> rest = full_string
      [unquote(num) | parse_both(rest)]
    end
  end
  defp parse_both(<<_>> <> rest), do: parse_both(rest)
  
  def run(input) do
    lines = input |> String.trim() |> String.split("\n")
    {p1, p2} = Enum.reduce(lines, {0, 0}, fn line, {t1, t2} ->
      nums1 = parse(line)
      nums2 = parse_both(line)
      if nums1 != [] && nums2 != [] do
        {t1 + (10 * List.first(nums1)) + List.last(nums1), t2 + (10 * List.first(nums2)) + List.last(nums2)}
      else
        {t1, t2}
      end
    end)

    IO.write("#{p1}\n#{p2}")
  end
end

Aoc.run(IO.read(:all))
