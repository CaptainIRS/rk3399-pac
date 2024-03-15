#[doc = "Register `TRANSMIT_TLP_COUNT` reader"]
pub type R = crate::R<TransmitTlpCountSpec>;
#[doc = "Register `TRANSMIT_TLP_COUNT` writer"]
pub type W = crate::W<TransmitTlpCountSpec>;
#[doc = "Field `TTC` reader - Transmit TLP Count \\[TTC\\]
Count of TLPs transmitted"]
pub type TtcR = crate::FieldReader<u32>;
#[doc = "Field `TTC` writer - Transmit TLP Count \\[TTC\\]
Count of TLPs transmitted"]
pub type TtcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit TLP Count \\[TTC\\]
Count of TLPs transmitted"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit TLP Count \\[TTC\\]
Count of TLPs transmitted"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TtcW<TransmitTlpCountSpec> {
        TtcW::new(self, 0)
    }
}
#[doc = "Transmit TLP Count Register Count of TLPs transmitted\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_tlp_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_tlp_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmitTlpCountSpec;
impl crate::RegisterSpec for TransmitTlpCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_tlp_count::R`](R) reader structure"]
impl crate::Readable for TransmitTlpCountSpec {}
#[doc = "`write(|w| ..)` method takes [`transmit_tlp_count::W`](W) writer structure"]
impl crate::Writable for TransmitTlpCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets TRANSMIT_TLP_COUNT to value 0"]
impl crate::Resettable for TransmitTlpCountSpec {
    const RESET_VALUE: u32 = 0;
}
