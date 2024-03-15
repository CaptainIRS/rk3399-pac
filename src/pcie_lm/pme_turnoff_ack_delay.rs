#[doc = "Register `PME_TURNOFF_ACK_DELAY` reader"]
pub type R = crate::R<PmeTurnoffAckDelaySpec>;
#[doc = "Register `PME_TURNOFF_ACK_DELAY` writer"]
pub type W = crate::W<PmeTurnoffAckDelaySpec>;
#[doc = "Field `PTOAD` reader - PME Turnoff Ack Delay \\[PTOAD\\]
Time in microseconds between the core receiving a PME_TurnOff message TLP and the core sending a PME_TO_Ack response to it. This field must be set to a non-zero value in order for the core to send a response. Setting this field to 0 suppresses the core's response to PME_TurnOff message, so that the client may transmit the PME_TO_Ack message through the master interface."]
pub type PtoadR = crate::FieldReader<u16>;
#[doc = "Field `PTOAD` writer - PME Turnoff Ack Delay \\[PTOAD\\]
Time in microseconds between the core receiving a PME_TurnOff message TLP and the core sending a PME_TO_Ack response to it. This field must be set to a non-zero value in order for the core to send a response. Setting this field to 0 suppresses the core's response to PME_TurnOff message, so that the client may transmit the PME_TO_Ack message through the master interface."]
pub type PtoadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R7` reader - Reserved \\[R7\\]
Reserved"]
pub type R7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PME Turnoff Ack Delay \\[PTOAD\\]
Time in microseconds between the core receiving a PME_TurnOff message TLP and the core sending a PME_TO_Ack response to it. This field must be set to a non-zero value in order for the core to send a response. Setting this field to 0 suppresses the core's response to PME_TurnOff message, so that the client may transmit the PME_TO_Ack message through the master interface."]
    #[inline(always)]
    pub fn ptoad(&self) -> PtoadR {
        PtoadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R7\\]
Reserved"]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PME Turnoff Ack Delay \\[PTOAD\\]
Time in microseconds between the core receiving a PME_TurnOff message TLP and the core sending a PME_TO_Ack response to it. This field must be set to a non-zero value in order for the core to send a response. Setting this field to 0 suppresses the core's response to PME_TurnOff message, so that the client may transmit the PME_TO_Ack message through the master interface."]
    #[inline(always)]
    #[must_use]
    pub fn ptoad(&mut self) -> PtoadW<PmeTurnoffAckDelaySpec> {
        PtoadW::new(self, 0)
    }
}
#[doc = "PME TurnOff Ack Delay Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pme_turnoff_ack_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pme_turnoff_ack_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmeTurnoffAckDelaySpec;
impl crate::RegisterSpec for PmeTurnoffAckDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pme_turnoff_ack_delay::R`](R) reader structure"]
impl crate::Readable for PmeTurnoffAckDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`pme_turnoff_ack_delay::W`](W) writer structure"]
impl crate::Writable for PmeTurnoffAckDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PME_TURNOFF_ACK_DELAY to value 0x64"]
impl crate::Resettable for PmeTurnoffAckDelaySpec {
    const RESET_VALUE: u32 = 0x64;
}
