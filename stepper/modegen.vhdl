library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity modegen is
  generic (
      width: integer := 10
  );
  port (
    clk : in std_logic;
    rst : in std_logic;
    mode : out std_logic_vector(1 downto 0)
  );
end modegen;

architecture rtl of modegen is
    signal counter : unsigned(width-1 downto 0);
begin
  proc : process (clk, rst)
  begin
    if rising_edge(clk) then
      if (rst = '1') then
        counter <= (others => '0');
      else
        counter <= counter+1;
      end if; -- rst
    end if; -- clk
  end process;
  mode <= std_logic_vector(counter(width-1 downto width-2));
end rtl;