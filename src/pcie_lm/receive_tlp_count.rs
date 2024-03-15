#[doc = "Register `RECEIVE_TLP_COUNT` reader"]
pub type R = crate::R<ReceiveTlpCountSpec>;
#[doc = "Register `RECEIVE_TLP_COUNT` writer"]
pub type W = crate::W<ReceiveTlpCountSpec>;
#[doc = "Field `RTC` reader - Receive TLP Count \\[RTC\\]
Count of TLPs received"]
pub type RtcR = crate::FieldReader<u32>;
#[doc = "Field `RTC` writer - Receive TLP Count \\[RTC\\]
Count of TLPs received"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive TLP Count \\[RTC\\]
Count of TLPs received"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive TLP Count \\[RTC\\]
Count of TLPs received"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<ReceiveTlpCountSpec> {
        RtcW::new(self, 0)
    }
}
#[doc = "Receive TLP Count Register Count of TLPs received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_tlp_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`receive_tlp_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveTlpCountSpec;
impl crate::RegisterSpec for ReceiveTlpCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_tlp_count::R`](R) reader structure"]
impl crate::Readable for ReceiveTlpCountSpec {}
#[doc = "`write(|w| ..)` method takes [`receive_tlp_count::W`](W) writer structure"]
impl crate::Writable for ReceiveTlpCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets RECEIVE_TLP_COUNT to value 0"]
impl crate::Resettable for ReceiveTlpCountSpec {
    const RESET_VALUE: u32 = 0;
}
