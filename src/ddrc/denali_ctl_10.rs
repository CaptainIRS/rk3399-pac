#[doc = "Register `DENALI_CTL_10` reader"]
pub type R = crate::R<DenaliCtl10Spec>;
#[doc = "Register `DENALI_CTL_10` writer"]
pub type W = crate::W<DenaliCtl10Spec>;
#[doc = "Field `TINIT3_F1` reader - DRAM TINIT3 value for frequency copy 1 in cycles."]
pub type Tinit3F1R = crate::FieldReader<u32>;
#[doc = "Field `TINIT3_F1` writer - DRAM TINIT3 value for frequency copy 1 in cycles."]
pub type Tinit3F1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT3 value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tinit3_f1(&self) -> Tinit3F1R {
        Tinit3F1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT3 value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit3_f1(&mut self) -> Tinit3F1W<DenaliCtl10Spec> {
        Tinit3F1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl10Spec;
impl crate::RegisterSpec for DenaliCtl10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_10::R`](R) reader structure"]
impl crate::Readable for DenaliCtl10Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_10::W`](W) writer structure"]
impl crate::Writable for DenaliCtl10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_10 to value 0"]
impl crate::Resettable for DenaliCtl10Spec {
    const RESET_VALUE: u32 = 0;
}
