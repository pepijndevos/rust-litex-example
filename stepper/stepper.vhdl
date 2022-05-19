library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity stepper is
  port (
    clk : in std_logic;
    rst : in std_logic;
    steps, period : in std_logic_vector(31 downto 0);
    strobe : in std_logic;
    H : out std_logic_vector(3 downto 0)
  );
end stepper;

architecture rtl of stepper is
    signal counter : unsigned(31 downto 0);
    signal step : std_logic;
    signal stepsrem : unsigned(31 downto 0);
    signal stepdir : std_logic;
    signal stepidx : unsigned(1 downto 0);
begin
  proc : process (clk, rst)
  begin
    if rising_edge(clk) then
      if (rst = '1') then
        counter <= (others => '0');
        step <= '0';
        stepsrem <= (others => '0');
        stepdir <= '0';
        stepidx <= (others => '0');
        H <= "0000";
      else
        -- generate step pulse
        if counter < unsigned(period) then
            counter <= counter + 1;
            step <= '0';
        else
            counter <= (others => '0');
            step <= '1';
            if stepdir = '1' then
                stepidx <= stepidx + 1;
            else
                stepidx <= stepidx - 1;
            end if;
        end if;

        -- set interal state on CPU write
        if (strobe = '1') then
            stepsrem <= unsigned(steps) when signed(steps) > 0 else unsigned(-signed(steps));
            stepdir <= '1' when signed(steps) > 0 else '0';
        end if;
        -- Step C0 C1 C2 C3
        --    1  1  0  1  0
        --    2  0  1  1  0
        --    3  0  1  0  1
        --    4  1  0  0  1 
        if (step = '1') then
            if (stepsrem > 0) then
                stepsrem <= stepsrem -1;
                case stepidx is
                    when "00" => H <= "1010";
                    when "01" => H <= "0110";
                    when "10" => H <= "0101";
                    when "11" => H <= "1001";
                    when others => H <= "0000";
                end case;
            else
                H <= "0000";
            end if;
        end if;
      end if;

    end if;
  end process;
  --   s <= i0 xor i1 xor ci;
  --   co <= (i0 and i1) or (i0 and ci) or (i1 and ci);
end rtl;