defmodule Lasagna do
  @doc """
  Methods for making a nice, tasty lasagna.
  """

  @expected_minutes_in_oven 40
  @minutes_per_layer 2

  @spec expected_minutes_in_oven :: integer()
  def expected_minutes_in_oven, do: @expected_minutes_in_oven

  @spec remaining_minutes_in_oven(integer()) :: integer()
  def remaining_minutes_in_oven(actual_minutes_in_oven),
    do: expected_minutes_in_oven() - actual_minutes_in_oven

  @spec preparation_time_in_minutes(integer()) :: integer()
  def preparation_time_in_minutes(number_of_layers), do: @minutes_per_layer * number_of_layers

  @spec total_time_in_minutes(integer(), integer()) :: integer()
  def total_time_in_minutes(number_of_layers, actual_minutes_in_oven),
    do: actual_minutes_in_oven + preparation_time_in_minutes(number_of_layers)

  @spec alarm() :: String.t()
  def alarm(), do: "Ding!"
end
