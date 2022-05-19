#
# This file is part of LiteX.
#
# Copyright (c) 2015-2019 Florent Kermarrec <florent@enjoy-digital.fr>
# SPDX-License-Identifier: BSD-2-Clause

from migen import *
from migen.genlib.cdc import MultiReg

from litex.soc.interconnect.csr import *

# Stepper motor driver ---------------------------------------------------------------------------

class Stepper(Module, AutoCSR):
    def __init__(self, H=None, clock_domain="sys", with_csr=True,
        default_steps  = 0,
        default_period = 0):
        if H is None:
            self.H = H = Signal(4)
        self.strobe = Signal()
        self.steps  = Signal(32, reset=default_steps)
        self.period = Signal(32, reset=default_period)

        self.specials += Instance("stepper",
            i_clk=ClockSignal(clock_domain),
            i_rst=ResetSignal(clock_domain),
            i_steps=self.steps,
            i_period=self.period,
            i_strobe=self.strobe,
            o_H=H
        )

        if with_csr:
            self.add_csr(clock_domain)

    def add_csr(self, clock_domain):
        self._steps  = CSRStorage(32, description="""Number of steps.\n
            Write this register to move the motor N steps.
            Negative numbers move the motor backwards.""",
            reset = self.steps.reset)
        self._period = CSRStorage(32, description="""Step period.\n
            Defines the period of the stepper in ``{cd}_clk`` cycles.""".format(cd=clock_domain),
            reset = self.period.reset)

        n = 0 if clock_domain == "sys" else 2
        self.specials += [
            MultiReg(self._steps.re, self.strobe, n=n),
            MultiReg(self._steps.storage,  self.steps,  n=n),
            MultiReg(self._period.storage, self.period, n=n),
        ]
