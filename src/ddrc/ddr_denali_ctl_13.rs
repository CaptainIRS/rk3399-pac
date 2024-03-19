#[doc = "Register `DDR_DENALI_CTL_13` reader"]
pub type R = crate::R<DdrDenaliCtl13Spec>;
#[doc = "Register `DDR_DENALI_CTL_13` writer"]
pub type W = crate::W<DdrDenaliCtl13Spec>;
#[doc = "Field `TINIT_F2` reader - DRAM TINIT value for frequency copy 2 in cycles."]
pub type TinitF2R = crate::FieldReader<u32>;
#[doc = "Field `TINIT_F2` writer - DRAM TINIT value for frequency copy 2 in cycles."]
pub type TinitF2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DRAM TINIT value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tinit_f2(&self) -> TinitF2R {
        TinitF2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DRAM TINIT value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tinit_f2(&mut self) -> TinitF2W<DdrDenaliCtl13Spec> {
        TinitF2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl13Spec;
impl crate::RegisterSpec for DdrDenaliCtl13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_13::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl13Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_13::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_13 to value 0"]
impl crate::Resettable for DdrDenaliCtl13Spec {
    const RESET_VALUE: u32 = 0;
}
