#[doc = "Register `PROBE_Counters_3_Src` reader"]
pub type R = crate::R<ProbeCounters3SrcSpec>;
#[doc = "Register `PROBE_Counters_3_Src` writer"]
pub type W = crate::W<ProbeCounters3SrcSpec>;
#[doc = "Field `INTEVENT` reader - Internal packet event\n\nEvent | source type | Event description\n\n5'h00 OFF Counter disabled.\n\n5'h01 CYCLE8 Probe clock cycles.\n\n5'h02 IDLE Idle cycles during which no packet data is\n\nobserved.\n\n5'h03 XFER Transfer cycles during which packet data is\n\ntransferred.\n\n5'h04 BUSY Busy cycles during which the packet data is\n\nmade available by the transmitting agent but the receiving agent is\n\nnot ready to receive it.\n\n5'h05 WAIT Wait cycles during a packet in which the\n\ntransmitting agent suspends the transfer of packet data.\n\n5'h06 PKT Packets.\n\n5'h08 BYTE Total number of payload bytes.\n\n5'h09 PRESS Clock cycles with pressure level > 0.\n\n5'h0A PRESS Clock cycles with pressure level > 1.\n\n5'h0B PRESS Clock cycles with pressure level > 2.\n\n5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
pub type InteventR = crate::FieldReader;
#[doc = "Field `INTEVENT` writer - Internal packet event\n\nEvent | source type | Event description\n\n5'h00 OFF Counter disabled.\n\n5'h01 CYCLE8 Probe clock cycles.\n\n5'h02 IDLE Idle cycles during which no packet data is\n\nobserved.\n\n5'h03 XFER Transfer cycles during which packet data is\n\ntransferred.\n\n5'h04 BUSY Busy cycles during which the packet data is\n\nmade available by the transmitting agent but the receiving agent is\n\nnot ready to receive it.\n\n5'h05 WAIT Wait cycles during a packet in which the\n\ntransmitting agent suspends the transfer of packet data.\n\n5'h06 PKT Packets.\n\n5'h08 BYTE Total number of payload bytes.\n\n5'h09 PRESS Clock cycles with pressure level > 0.\n\n5'h0A PRESS Clock cycles with pressure level > 1.\n\n5'h0B PRESS Clock cycles with pressure level > 2.\n\n5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
pub type InteventW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Internal packet event\n\nEvent | source type | Event description\n\n5'h00 OFF Counter disabled.\n\n5'h01 CYCLE8 Probe clock cycles.\n\n5'h02 IDLE Idle cycles during which no packet data is\n\nobserved.\n\n5'h03 XFER Transfer cycles during which packet data is\n\ntransferred.\n\n5'h04 BUSY Busy cycles during which the packet data is\n\nmade available by the transmitting agent but the receiving agent is\n\nnot ready to receive it.\n\n5'h05 WAIT Wait cycles during a packet in which the\n\ntransmitting agent suspends the transfer of packet data.\n\n5'h06 PKT Packets.\n\n5'h08 BYTE Total number of payload bytes.\n\n5'h09 PRESS Clock cycles with pressure level > 0.\n\n5'h0A PRESS Clock cycles with pressure level > 1.\n\n5'h0B PRESS Clock cycles with pressure level > 2.\n\n5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
    #[inline(always)]
    pub fn intevent(&self) -> InteventR {
        InteventR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Internal packet event\n\nEvent | source type | Event description\n\n5'h00 OFF Counter disabled.\n\n5'h01 CYCLE8 Probe clock cycles.\n\n5'h02 IDLE Idle cycles during which no packet data is\n\nobserved.\n\n5'h03 XFER Transfer cycles during which packet data is\n\ntransferred.\n\n5'h04 BUSY Busy cycles during which the packet data is\n\nmade available by the transmitting agent but the receiving agent is\n\nnot ready to receive it.\n\n5'h05 WAIT Wait cycles during a packet in which the\n\ntransmitting agent suspends the transfer of packet data.\n\n5'h06 PKT Packets.\n\n5'h08 BYTE Total number of payload bytes.\n\n5'h09 PRESS Clock cycles with pressure level > 0.\n\n5'h0A PRESS Clock cycles with pressure level > 1.\n\n5'h0B PRESS Clock cycles with pressure level > 2.\n\n5'h10 CHAIN Carry from counter 2m to counter 2m + 1."]
    #[inline(always)]
    #[must_use]
    pub fn intevent(&mut self) -> InteventW<ProbeCounters3SrcSpec> {
        InteventW::new(self, 0)
    }
}
#[doc = "Register CntSrc indicates the event source.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_3_src::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_counters_3_src::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeCounters3SrcSpec;
impl crate::RegisterSpec for ProbeCounters3SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_counters_3_src::R`](R) reader structure"]
impl crate::Readable for ProbeCounters3SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`probe_counters_3_src::W`](W) writer structure"]
impl crate::Writable for ProbeCounters3SrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROBE_Counters_3_Src to value 0"]
impl crate::Resettable for ProbeCounters3SrcSpec {
    const RESET_VALUE: u32 = 0;
}
