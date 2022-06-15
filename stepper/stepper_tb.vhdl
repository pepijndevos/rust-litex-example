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
    generic (
        delay: integer := 10
    );
    port (
        clk : in std_logic;
        rst : in std_logic;
        steps, period : in std_logic_vector(31 downto 0);
        steps_wr : in std_logic;
        mode : in std_logic_vector(1 downto 0);
        step_out : out std_logic
    );
  end component;

  component modegen
    generic (
        width: integer := 10
    );
    port (
        clk : in std_logic;
        rst : in std_logic;
        mode : out std_logic_vector(1 downto 0)
    );
  end component;

  --  Specifies which entity is bound with the component.
  for stepper_0: stepper use entity work.stepper;
  signal rst, clk, steps_wr, step_out : std_logic := '0';
  signal steps, period : std_logic_vector(31 downto 0) := (others => '0');
  signal mode : std_logic_vector(1 downto 0) := "00";
begin
  --  Component instantiation.
  stepper_0: stepper generic map (delay => 3) port map (clk => clk, rst => rst, steps => steps, period => period, steps_wr => steps_wr, mode => mode, step_out => step_out);
  modegen_0: modegen generic map (width => 5) port map (clk => clk, rst => rst, mode => mode);

  clk <= not clk after 1 ns;
  --  This process does the real job.
  process
  begin
    rst <= '1';
    wait for 1 ns;
    rst <= '0';
    steps <= std_logic_vector(to_signed(5, 32));
    period <= std_logic_vector(to_unsigned(3, 32));
    steps_wr <= '1';
    wait for 2 ns;
    steps_wr <= '0';

    wait for 2000 ns;
    steps <= std_logic_vector(to_signed(-5, 32));
    period <= std_logic_vector(to_unsigned(3, 32));
    steps_wr <= '1';
    wait for 2 ns;
    steps_wr <= '0';

    wait for 2000 ns;
    finish;
  end process;

end behav;