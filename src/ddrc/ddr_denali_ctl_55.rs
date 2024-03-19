#[doc = "Register `DDR_DENALI_CTL_55` reader"]
pub type R = crate::R<DdrDenaliCtl55Spec>;
#[doc = "Register `DDR_DENALI_CTL_55` writer"]
pub type W = crate::W<DdrDenaliCtl55Spec>;
#[doc = "Field `TMRRI_F0` reader - DRAM TMRRI value for frequency copy 0 in cycles."]
pub type TmrriF0R = crate::FieldReader;
#[doc = "Field `TMRRI_F0` writer - DRAM TMRRI value for frequency copy 0 in cycles."]
pub type TmrriF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRRI_F1` reader - DRAM TMRRI value for frequency copy 1 in cycles."]
pub type TmrriF1R = crate::FieldReader;
#[doc = "Field `TMRRI_F1` writer - DRAM TMRRI value for frequency copy 1 in cycles."]
pub type TmrriF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMRRI_F2` reader - DRAM TMRRI value for frequency copy 2 in cycles."]
pub type TmrriF2R = crate::FieldReader;
#[doc = "Field `TMRRI_F2` writer - DRAM TMRRI value for frequency copy 2 in cycles."]
pub type TmrriF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCSCKE_F0` reader - DRAM TCSCKE value for frequency copy 0 in cycles."]
pub type TcsckeF0R = crate::FieldReader;
#[doc = "Field `TCSCKE_F0` writer - DRAM TCSCKE value for frequency copy 0 in cycles."]
pub type TcsckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - DRAM TMRRI value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tmrri_f0(&self) -> TmrriF0R {
        TmrriF0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TMRRI value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tmrri_f1(&self) -> TmrriF1R {
        TmrriF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DRAM TMRRI value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tmrri_f2(&self) -> TmrriF2R {
        TmrriF2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCSCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tcscke_f0(&self) -> TcsckeF0R {
        TcsckeF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DRAM TMRRI value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrri_f0(&mut self) -> TmrriF0W<DdrDenaliCtl55Spec> {
        TmrriF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TMRRI value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrri_f1(&mut self) -> TmrriF1W<DdrDenaliCtl55Spec> {
        TmrriF1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - DRAM TMRRI value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrri_f2(&mut self) -> TmrriF2W<DdrDenaliCtl55Spec> {
        TmrriF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCSCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcscke_f0(&mut self) -> TcsckeF0W<DdrDenaliCtl55Spec> {
        TcsckeF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_55::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_55::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl55Spec;
impl crate::RegisterSpec for DdrDenaliCtl55Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_55::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl55Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_55::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_55 to value 0"]
impl crate::Resettable for DdrDenaliCtl55Spec {
    const RESET_VALUE: u32 = 0;
}
