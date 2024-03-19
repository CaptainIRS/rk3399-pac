#[doc = "Register `DDR_DENALI_CTL_37` reader"]
pub type R = crate::R<DdrDenaliCtl37Spec>;
#[doc = "Register `DDR_DENALI_CTL_37` writer"]
pub type W = crate::W<DdrDenaliCtl37Spec>;
#[doc = "Field `TRAS_MAX_F2` reader - DRAM TRAS_MAX value for frequency copy 2 in cycles."]
pub type TrasMaxF2R = crate::FieldReader<u32>;
#[doc = "Field `TRAS_MAX_F2` writer - DRAM TRAS_MAX value for frequency copy 2 in cycles."]
pub type TrasMaxF2W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `TCKE_F2` reader - Minimum CKE pulse width for frequency copy 2."]
pub type TckeF2R = crate::FieldReader;
#[doc = "Field `TCKE_F2` writer - Minimum CKE pulse width for frequency copy 2."]
pub type TckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:16 - DRAM TRAS_MAX value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tras_max_f2(&self) -> TrasMaxF2R {
        TrasMaxF2R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:27 - Minimum CKE pulse width for frequency copy 2."]
    #[inline(always)]
    pub fn tcke_f2(&self) -> TckeF2R {
        TckeF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - DRAM TRAS_MAX value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tras_max_f2(&mut self) -> TrasMaxF2W<DdrDenaliCtl37Spec> {
        TrasMaxF2W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Minimum CKE pulse width for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn tcke_f2(&mut self) -> TckeF2W<DdrDenaliCtl37Spec> {
        TckeF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_37::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_37::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl37Spec;
impl crate::RegisterSpec for DdrDenaliCtl37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_37::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl37Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_37::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_37 to value 0"]
impl crate::Resettable for DdrDenaliCtl37Spec {
    const RESET_VALUE: u32 = 0;
}
