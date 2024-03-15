#[doc = "Register `PROBE_Counters_0_Src` reader"]
pub type R = crate::R<ProbeCounters0SrcSpec>;
#[doc = "Register `PROBE_Counters_0_Src` writer"]
pub type W = crate::W<ProbeCounters0SrcSpec>;
#[doc = "Field `INTEVENT` reader - Internal packet event 5'h00 OFF Counter disabled. 5'h01 CYCLE8 Probe clock cycles. 5'h02 IDLE Idle cycles during which no packet data is observed. 5'h03 XFER Transfer cycles during which packet data is transferred. 5'h04 BUSY Busy cycles during which the packet data is made available by the transmitting agent but the receiving agent is not ready to receive it. 5'h05 WAIT Wait cycles during a packet in which the transmitting agent suspends the transfer of packet data. 5'h06 PKT Packets. 5'h08 BYTE Total number of payload bytes. 5'h09 PRESS Clock cycles with pressure level > 0. 5'h0A PRESS Clock cycles with pressure level > 1. 5'h0B PRESS Clock cycles with pressure level > 2. 5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
pub type InteventR = crate::FieldReader;
#[doc = "Field `INTEVENT` writer - Internal packet event 5'h00 OFF Counter disabled. 5'h01 CYCLE8 Probe clock cycles. 5'h02 IDLE Idle cycles during which no packet data is observed. 5'h03 XFER Transfer cycles during which packet data is transferred. 5'h04 BUSY Busy cycles during which the packet data is made available by the transmitting agent but the receiving agent is not ready to receive it. 5'h05 WAIT Wait cycles during a packet in which the transmitting agent suspends the transfer of packet data. 5'h06 PKT Packets. 5'h08 BYTE Total number of payload bytes. 5'h09 PRESS Clock cycles with pressure level > 0. 5'h0A PRESS Clock cycles with pressure level > 1. 5'h0B PRESS Clock cycles with pressure level > 2. 5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
pub type InteventW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Internal packet event 5'h00 OFF Counter disabled. 5'h01 CYCLE8 Probe clock cycles. 5'h02 IDLE Idle cycles during which no packet data is observed. 5'h03 XFER Transfer cycles during which packet data is transferred. 5'h04 BUSY Busy cycles during which the packet data is made available by the transmitting agent but the receiving agent is not ready to receive it. 5'h05 WAIT Wait cycles during a packet in which the transmitting agent suspends the transfer of packet data. 5'h06 PKT Packets. 5'h08 BYTE Total number of payload bytes. 5'h09 PRESS Clock cycles with pressure level > 0. 5'h0A PRESS Clock cycles with pressure level > 1. 5'h0B PRESS Clock cycles with pressure level > 2. 5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
    #[inline(always)]
    pub fn intevent(&self) -> InteventR {
        InteventR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Internal packet event 5'h00 OFF Counter disabled. 5'h01 CYCLE8 Probe clock cycles. 5'h02 IDLE Idle cycles during which no packet data is observed. 5'h03 XFER Transfer cycles during which packet data is transferred. 5'h04 BUSY Busy cycles during which the packet data is made available by the transmitting agent but the receiving agent is not ready to receive it. 5'h05 WAIT Wait cycles during a packet in which the transmitting agent suspends the transfer of packet data. 5'h06 PKT Packets. 5'h08 BYTE Total number of payload bytes. 5'h09 PRESS Clock cycles with pressure level > 0. 5'h0A PRESS Clock cycles with pressure level > 1. 5'h0B PRESS Clock cycles with pressure level > 2. 5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
    #[inline(always)]
    #[must_use]
    pub fn intevent(&mut self) -> InteventW<ProbeCounters0SrcSpec> {
        InteventW::new(self, 0)
    }
}
#[doc = "Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_0_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_counters_0_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeCounters0SrcSpec;
impl crate::RegisterSpec for ProbeCounters0SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_counters_0_src::R`](R) reader structure"]
impl crate::Readable for ProbeCounters0SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`probe_counters_0_src::W`](W) writer structure"]
impl crate::Writable for ProbeCounters0SrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROBE_Counters_0_Src to value 0"]
impl crate::Resettable for ProbeCounters0SrcSpec {
    const RESET_VALUE: u32 = 0;
}
