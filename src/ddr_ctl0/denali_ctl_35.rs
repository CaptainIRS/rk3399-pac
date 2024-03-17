#[doc = "Register `DENALI_CTL_35` reader"]
pub type R = crate::R<DenaliCtl35Spec>;
#[doc = "Register `DENALI_CTL_35` writer"]
pub type W = crate::W<DenaliCtl35Spec>;
#[doc = "Field `TRAS_MAX_F1` reader - DRAM TRAS_MAX value for frequency copy 1 in cycles."]
pub type TrasMaxF1R = crate::FieldReader<u32>;
#[doc = "Field `TRAS_MAX_F1` writer - DRAM TRAS_MAX value for frequency copy 1 in cycles."]
pub type TrasMaxF1W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `TCKE_F1` reader - Minimum CKE pulse width for frequency copy 1."]
pub type TckeF1R = crate::FieldReader;
#[doc = "Field `TCKE_F1` writer - Minimum CKE pulse width for frequency copy 1."]
pub type TckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:16 - DRAM TRAS_MAX value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tras_max_f1(&self) -> TrasMaxF1R {
        TrasMaxF1R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:27 - Minimum CKE pulse width for frequency copy 1."]
    #[inline(always)]
    pub fn tcke_f1(&self) -> TckeF1R {
        TckeF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - DRAM TRAS_MAX value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tras_max_f1(&mut self) -> TrasMaxF1W<DenaliCtl35Spec> {
        TrasMaxF1W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Minimum CKE pulse width for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn tcke_f1(&mut self) -> TckeF1W<DenaliCtl35Spec> {
        TckeF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_35::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_35::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl35Spec;
impl crate::RegisterSpec for DenaliCtl35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_35::R`](R) reader structure"]
impl crate::Readable for DenaliCtl35Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_35::W`](W) writer structure"]
impl crate::Writable for DenaliCtl35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_35 to value 0"]
impl crate::Resettable for DenaliCtl35Spec {
    const RESET_VALUE: u32 = 0;
}
