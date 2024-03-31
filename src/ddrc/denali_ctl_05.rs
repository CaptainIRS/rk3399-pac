#[doc = "Register `DENALI_CTL_05` reader"]
pub type R = crate::R<DenaliCtl05Spec>;
#[doc = "Register `DENALI_CTL_05` writer"]
pub type W = crate::W<DenaliCtl05Spec>;
#[doc = "Field `TINIT_F0` reader - DRAM TINIT value for frequency copy 0 in cycles."]
pub type TinitF0R = crate::FieldReader<u32>;
#[doc = "Field `TINIT_F0` writer - DRAM TINIT value for frequency copy 0 in cycles."]
pub type TinitF0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tinit_f0(&self) -> TinitF0R {
        TinitF0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit_f0(&mut self) -> TinitF0W<DenaliCtl05Spec> {
        TinitF0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_05::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_05::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl05Spec;
impl crate::RegisterSpec for DenaliCtl05Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_05::R`](R) reader structure"]
impl crate::Readable for DenaliCtl05Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_05::W`](W) writer structure"]
impl crate::Writable for DenaliCtl05Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_05 to value 0"]
impl crate::Resettable for DenaliCtl05Spec {
    const RESET_VALUE: u32 = 0;
}
