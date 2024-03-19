#[doc = "Register `PCIE_LM_RECEIVE_TLP_COUNT` reader"]
pub type R = crate::R<PcieLmReceiveTlpCountSpec>;
#[doc = "Register `PCIE_LM_RECEIVE_TLP_COUNT` writer"]
pub type W = crate::W<PcieLmReceiveTlpCountSpec>;
#[doc = "Field `RTC` reader - Receive TLP Count \\[RTC\\]\n\nCount of TLPs received"]
pub type RtcR = crate::FieldReader<u32>;
#[doc = "Field `RTC` writer - Receive TLP Count \\[RTC\\]\n\nCount of TLPs received"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive TLP Count \\[RTC\\]\n\nCount of TLPs received"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive TLP Count \\[RTC\\]\n\nCount of TLPs received"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<PcieLmReceiveTlpCountSpec> {
        RtcW::new(self, 0)
    }
}
#[doc = "Receive TLP Count Register\n\nCount of TLPs received\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_receive_tlp_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_receive_tlp_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmReceiveTlpCountSpec;
impl crate::RegisterSpec for PcieLmReceiveTlpCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_receive_tlp_count::R`](R) reader structure"]
impl crate::Readable for PcieLmReceiveTlpCountSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_receive_tlp_count::W`](W) writer structure"]
impl crate::Writable for PcieLmReceiveTlpCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets PCIE_LM_RECEIVE_TLP_COUNT to value 0"]
impl crate::Resettable for PcieLmReceiveTlpCountSpec {
    const RESET_VALUE: u32 = 0;
}
