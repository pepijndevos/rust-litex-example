library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use std.env.finish;

--  A testbench has no ports.
entity stepper_tb is
end stepper_tb;

architecture behav of stepper_tb is
  --  Declaration of the component that will be instantiated.
  component stepper
    port (
      clk : in std_logic;
      rst : in std_logic;
      steps, period : in std_logic_vector(31 downto 0);
      strobe : in std_logic;
      H : out std_logic_vector(3 downto 0)
    );
  end component;

  --  Specifies which entity is bound with the component.
  for stepper_0: stepper use entity work.stepper;
  signal rst, clk, strobe : std_logic := '0';
  signal steps, period : std_logic_vector(31 downto 0) := (others => '0');
  signal H : std_logic_vector(3 downto 0) := (others => '0');
begin
  --  Component instantiation.
  stepper_0: stepper port map (clk => clk, rst => rst, steps => steps, period => period, strobe => strobe, H => H);

  clk <= not clk after 1 ns;
  --  This process does the real job.
  process
  begin
    rst <= '1';
    wait for 1 ns;
    rst <= '0';
    steps <= std_logic_vector(to_signed(5, 32));
    period <= std_logic_vector(to_unsigned(3, 32));
    strobe <= '1';
    wait for 2 ns;
    strobe <= '0';

    wait for 50 ns;
    steps <= std_logic_vector(to_signed(-5, 32));
    period <= std_logic_vector(to_unsigned(3, 32));
    strobe <= '1';
    wait for 2 ns;
    strobe <= '0';

    wait for 50 ns;
    finish;
  end process;

end behav;