#[doc = "Register `PMUPVTM_STATUS0` reader"]
pub type R = crate::R<PmupvtmStatus0Spec>;
#[doc = "Register `PMUPVTM_STATUS0` writer"]
pub type W = crate::W<PmupvtmStatus0Spec>;
#[doc = "Field `PVTM_FREQ_DONE` reader - pvtm frequency calculate done status"]
pub type PvtmFreqDoneR = crate::BitReader;
#[doc = "Field `PVTM_FREQ_DONE` writer - pvtm frequency calculate done status"]
pub type PvtmFreqDoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - pvtm frequency calculate done status"]
    #[inline(always)]
    pub fn pvtm_freq_done(&self) -> PvtmFreqDoneR {
        PvtmFreqDoneR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - pvtm frequency calculate done status"]
    #[inline(always)]
    #[must_use]
    pub fn pvtm_freq_done(&mut self) -> PvtmFreqDoneW<PmupvtmStatus0Spec> {
        PvtmFreqDoneW::new(self, 0)
    }
}
#[doc = "pmu pvtm status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmupvtm_status0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmupvtm_status0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmupvtmStatus0Spec;
impl crate::RegisterSpec for PmupvtmStatus0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmupvtm_status0::R`](R) reader structure"]
impl crate::Readable for PmupvtmStatus0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmupvtm_status0::W`](W) writer structure"]
impl crate::Writable for PmupvtmStatus0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUPVTM_STATUS0 to value 0"]
impl crate::Resettable for PmupvtmStatus0Spec {
    const RESET_VALUE: u32 = 0;
}
