#[doc = "Register `DENALI_CTL_33` reader"]
pub type R = crate::R<DenaliCtl33Spec>;
#[doc = "Register `DENALI_CTL_33` writer"]
pub type W = crate::W<DenaliCtl33Spec>;
#[doc = "Field `TRAS_MAX_F0` reader - DRAM TRAS_MAX value for frequency copy 0 in cycles."]
pub type TrasMaxF0R = crate::FieldReader<u32>;
#[doc = "Field `TRAS_MAX_F0` writer - DRAM TRAS_MAX value for frequency copy 0 in cycles."]
pub type TrasMaxF0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `TCKE_F0` reader - Minimum CKE pulse width for frequency copy 0."]
pub type TckeF0R = crate::FieldReader;
#[doc = "Field `TCKE_F0` writer - Minimum CKE pulse width for frequency copy 0."]
pub type TckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:16 - DRAM TRAS_MAX value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tras_max_f0(&self) -> TrasMaxF0R {
        TrasMaxF0R::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bits 24:27 - Minimum CKE pulse width for frequency copy 0."]
    #[inline(always)]
    pub fn tcke_f0(&self) -> TckeF0R {
        TckeF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:16 - DRAM TRAS_MAX value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tras_max_f0(&mut self) -> TrasMaxF0W<DenaliCtl33Spec> {
        TrasMaxF0W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Minimum CKE pulse width for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn tcke_f0(&mut self) -> TckeF0W<DenaliCtl33Spec> {
        TckeF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl33Spec;
impl crate::RegisterSpec for DenaliCtl33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_33::R`](R) reader structure"]
impl crate::Readable for DenaliCtl33Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_33::W`](W) writer structure"]
impl crate::Writable for DenaliCtl33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_33 to value 0"]
impl crate::Resettable for DenaliCtl33Spec {
    const RESET_VALUE: u32 = 0;
}
