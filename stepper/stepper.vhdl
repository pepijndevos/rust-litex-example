library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity stepper is
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
end stepper;

architecture rtl of stepper is
    type motor_state_t is (IDLE, INIT, FWD, REV);
    signal motor_state : motor_state_t := IDLE;
    signal stepsign : boolean;
    signal stepsrem : unsigned(31 downto 0) := (others => '0');
    signal counter : unsigned(31 downto 0) := (others => '0');
    signal mode_en : std_logic := '0';
    signal mode_sr : std_logic_vector(delay downto 0) := (others => '0');
begin
  proc : process (clk, rst)
  begin
    if rising_edge(clk) then
      if (rst = '1') then
        step_out <= '0';
        stepsrem <= unsigned(steps) when stepsign else unsigned(-signed(steps));
        motor_state <= IDLE when stepsign else INIT;
      else
        -- create a pulse when mode LSB changes
        mode_sr <= mode_sr(mode_sr'high-1 downto 0) & mode(0);

        -- a mode change happened
        if mode_en then
            -- increment the counter
            if (motor_state = INIT and mode = "11") then
                step_out <= '1';
                motor_state <= FWD when stepsign else REV;
            elsif (motor_state = FWD  and mode = "10")
                or (motor_state = REV  and mode = "01") then
                if counter < unsigned(period) then
                    counter <= counter + 1;
                    step_out <= '0';
                else -- reset the counter and perform a step
                    counter <= (others => '0');
                    step_out <= '1';
                    stepsrem <= stepsrem -1;
                end if;
            else
                step_out <= '0';
            end if;
                
            if (stepsrem = 0) then
                motor_state <= IDLE;
            end if;
        end if; -- mode_en


        -- set interal state on CPU write
        if (steps_wr = '1') then
            stepsrem <= unsigned(steps) when stepsign else unsigned(-signed(steps));
            motor_state <= INIT;
        end if; -- step_wr
      end if; -- rst
    end if; -- clk
  end process;
  mode_en <= mode_sr(mode_sr'high) xor mode_sr(mode_sr'high-1);
  stepsign <= steps(31) = '0';
end rtl;