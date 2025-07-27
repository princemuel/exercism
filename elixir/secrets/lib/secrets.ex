defmodule Secrets do
  @spec secret_add(number()) :: (number() -> number())
  def secret_add(secret) do
    fn x -> x + secret end
  end

  @spec secret_subtract(number()) :: (number() -> number())
  def secret_subtract(secret) do
    fn x -> x - secret end
  end

  @spec secret_multiply(number()) :: (number() -> number())
  def secret_multiply(secret) do
    fn x -> x * secret end
  end

  @spec secret_divide(number()) :: (number() -> number())
  def secret_divide(secret) do
    fn x -> div(x, secret) end
  end

  @spec secret_and(number()) :: (number() -> number())
  def secret_and(secret) do
    fn x -> Bitwise.band(x, secret) end
  end

  @spec secret_xor(number()) :: (number() -> number())
  def secret_xor(secret) do
    fn x -> Bitwise.bxor(x, secret) end
  end

  @spec secret_combine((a -> b), (b -> c)) :: (a -> c) when a: any(), b: any(), c: any()
  def secret_combine(secret_function1, secret_function2) do
    fn x -> secret_function2.(secret_function1.(x)) end
  end
end
