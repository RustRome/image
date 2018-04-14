defmodule Image do
  use Rustler, otp_app: :image, crate: "img"

  def add(_x,_y), do: err()

  def flip(_source,_dest), do: err()

  defp err() do
    throw(NifNotLoadedError)
  end
end
