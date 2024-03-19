#[doc = "Register `UART_MCR` reader"]
pub type R = crate::R<UartMcrSpec>;
#[doc = "Register `UART_MCR` writer"]
pub type W = crate::W<UartMcrSpec>;
#[doc = "Field `DATA_TERMINAL_READY` reader - Data Terminal Ready.\n\nThis is used to directly control the Data Terminal Ready (dtr_n)\n\noutput. The value written to this location is inverted and driven\n\nout on dtr_n, that is:\n\n0 = dtr_n de-asserted (logic 1)\n\n1 = dtr_n asserted (logic 0)"]
pub type DataTerminalReadyR = crate::BitReader;
#[doc = "Field `DATA_TERMINAL_READY` writer - Data Terminal Ready.\n\nThis is used to directly control the Data Terminal Ready (dtr_n)\n\noutput. The value written to this location is inverted and driven\n\nout on dtr_n, that is:\n\n0 = dtr_n de-asserted (logic 1)\n\n1 = dtr_n asserted (logic 0)"]
pub type DataTerminalReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQ_TO_SEND` reader - Request to Send.\n\n\n\nThis is used to directly control the Request to Send (rts_n)\n\noutput. The Request To Send (rts_n) output is used to inform the\n\nmodem or data set that the UART is ready to exchange data."]
pub type ReqToSendR = crate::BitReader;
#[doc = "Field `REQ_TO_SEND` writer - Request to Send.\n\n\n\nThis is used to directly control the Request to Send (rts_n)\n\noutput. The Request To Send (rts_n) output is used to inform the\n\nmodem or data set that the UART is ready to exchange data."]
pub type ReqToSendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1` reader - OUT1\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n1’b0: out2_n de-asserted (logic 1)\n\n1’b1: out2_n asserted (logic 0)"]
pub type Out1R = crate::BitReader;
#[doc = "Field `OUT1` writer - OUT1\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n1’b0: out2_n de-asserted (logic 1)\n\n1’b1: out2_n asserted (logic 0)"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2` reader - OUT2.\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n0 = out2_n de-asserted (logic 1)\n\n1 = out2_n asserted (logic 0)"]
pub type Out2R = crate::BitReader;
#[doc = "Field `OUT2` writer - OUT2.\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n0 = out2_n de-asserted (logic 1)\n\n1 = out2_n asserted (logic 0)"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - LoopBack Bit.\n\nThis is used to put the UART into a diagnostic mode for test\n\npurposes."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - LoopBack Bit.\n\nThis is used to put the UART into a diagnostic mode for test\n\npurposes."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_FLOW_CTRL_EN` reader - Auto Flow Control Enable.\n\n0 = Auto Flow Control Mode disabled\n\n1 = Auto Flow Control Mode enabled"]
pub type AutoFlowCtrlEnR = crate::BitReader;
#[doc = "Field `AUTO_FLOW_CTRL_EN` writer - Auto Flow Control Enable.\n\n0 = Auto Flow Control Mode disabled\n\n1 = Auto Flow Control Mode enabled"]
pub type AutoFlowCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIR_MODE_EN` reader - SIR Mode Enable.\n\nSIR Mode Enable.\n\nThis is used to enable/disable the IrDA SIR Mode .\n\n0 = IrDA SIR Mode disabled\n\n1 = IrDA SIR Mode enabled"]
pub type SirModeEnR = crate::BitReader;
#[doc = "Field `SIR_MODE_EN` writer - SIR Mode Enable.\n\nSIR Mode Enable.\n\nThis is used to enable/disable the IrDA SIR Mode .\n\n0 = IrDA SIR Mode disabled\n\n1 = IrDA SIR Mode enabled"]
pub type SirModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data Terminal Ready.\n\nThis is used to directly control the Data Terminal Ready (dtr_n)\n\noutput. The value written to this location is inverted and driven\n\nout on dtr_n, that is:\n\n0 = dtr_n de-asserted (logic 1)\n\n1 = dtr_n asserted (logic 0)"]
    #[inline(always)]
    pub fn data_terminal_ready(&self) -> DataTerminalReadyR {
        DataTerminalReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to Send.\n\n\n\nThis is used to directly control the Request to Send (rts_n)\n\noutput. The Request To Send (rts_n) output is used to inform the\n\nmodem or data set that the UART is ready to exchange data."]
    #[inline(always)]
    pub fn req_to_send(&self) -> ReqToSendR {
        ReqToSendR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OUT1\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n1’b0: out2_n de-asserted (logic 1)\n\n1’b1: out2_n asserted (logic 0)"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OUT2.\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n0 = out2_n de-asserted (logic 1)\n\n1 = out2_n asserted (logic 0)"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LoopBack Bit.\n\nThis is used to put the UART into a diagnostic mode for test\n\npurposes."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable.\n\n0 = Auto Flow Control Mode disabled\n\n1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    pub fn auto_flow_ctrl_en(&self) -> AutoFlowCtrlEnR {
        AutoFlowCtrlEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SIR Mode Enable.\n\nSIR Mode Enable.\n\nThis is used to enable/disable the IrDA SIR Mode .\n\n0 = IrDA SIR Mode disabled\n\n1 = IrDA SIR Mode enabled"]
    #[inline(always)]
    pub fn sir_mode_en(&self) -> SirModeEnR {
        SirModeEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data Terminal Ready.\n\nThis is used to directly control the Data Terminal Ready (dtr_n)\n\noutput. The value written to this location is inverted and driven\n\nout on dtr_n, that is:\n\n0 = dtr_n de-asserted (logic 1)\n\n1 = dtr_n asserted (logic 0)"]
    #[inline(always)]
    #[must_use]
    pub fn data_terminal_ready(&mut self) -> DataTerminalReadyW<UartMcrSpec> {
        DataTerminalReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - Request to Send.\n\n\n\nThis is used to directly control the Request to Send (rts_n)\n\noutput. The Request To Send (rts_n) output is used to inform the\n\nmodem or data set that the UART is ready to exchange data."]
    #[inline(always)]
    #[must_use]
    pub fn req_to_send(&mut self) -> ReqToSendW<UartMcrSpec> {
        ReqToSendW::new(self, 1)
    }
    #[doc = "Bit 2 - OUT1\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n1’b0: out2_n de-asserted (logic 1)\n\n1’b1: out2_n asserted (logic 0)"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<UartMcrSpec> {
        Out1W::new(self, 2)
    }
    #[doc = "Bit 3 - OUT2.\n\nThis is used to directly control the user-designated Output2\n\n(out2_n) output. The value written to this location is inverted and\n\ndriven out on out2_n, that is:\n\n0 = out2_n de-asserted (logic 1)\n\n1 = out2_n asserted (logic 0)"]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<UartMcrSpec> {
        Out2W::new(self, 3)
    }
    #[doc = "Bit 4 - LoopBack Bit.\n\nThis is used to put the UART into a diagnostic mode for test\n\npurposes."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LoopbackW<UartMcrSpec> {
        LoopbackW::new(self, 4)
    }
    #[doc = "Bit 5 - Auto Flow Control Enable.\n\n0 = Auto Flow Control Mode disabled\n\n1 = Auto Flow Control Mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn auto_flow_ctrl_en(&mut self) -> AutoFlowCtrlEnW<UartMcrSpec> {
        AutoFlowCtrlEnW::new(self, 5)
    }
    #[doc = "Bit 6 - SIR Mode Enable.\n\nSIR Mode Enable.\n\nThis is used to enable/disable the IrDA SIR Mode .\n\n0 = IrDA SIR Mode disabled\n\n1 = IrDA SIR Mode enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sir_mode_en(&mut self) -> SirModeEnW<UartMcrSpec> {
        SirModeEnW::new(self, 6)
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_mcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_mcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartMcrSpec;
impl crate::RegisterSpec for UartMcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_mcr::R`](R) reader structure"]
impl crate::Readable for UartMcrSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_mcr::W`](W) writer structure"]
impl crate::Writable for UartMcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_MCR to value 0"]
impl crate::Resettable for UartMcrSpec {
    const RESET_VALUE: u32 = 0;
}
