#[doc = "Register `DDR_DENALI_CTL_58` reader"]
pub type R = crate::R<DdrDenaliCtl58Spec>;
#[doc = "Register `DDR_DENALI_CTL_58` writer"]
pub type W = crate::W<DdrDenaliCtl58Spec>;
#[doc = "Field `TZQCKE_F1` reader - DRAM TZQCKE value for frequency copy 1 in cycles."]
pub type TzqckeF1R = crate::FieldReader;
#[doc = "Field `TZQCKE_F1` writer - DRAM TZQCKE value for frequency copy 1 in cycles."]
pub type TzqckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCSCKE_F2` reader - DRAM TCSCKE value for frequency copy 2 in cycles."]
pub type TcsckeF2R = crate::FieldReader;
#[doc = "Field `TCSCKE_F2` writer - DRAM TCSCKE value for frequency copy 2 in cycles."]
pub type TcsckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKELCS_F2` reader - DRAM TCKELCS value for frequency copy 2 in cycles."]
pub type TckelcsF2R = crate::FieldReader;
#[doc = "Field `TCKELCS_F2` writer - DRAM TCKELCS value for frequency copy 2 in cycles."]
pub type TckelcsF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKEHCS_F2` reader - DRAM TCKEHCS value for frequency copy 2 in cycles."]
pub type TckehcsF2R = crate::FieldReader;
#[doc = "Field `TCKEHCS_F2` writer - DRAM TCKEHCS value for frequency copy 2 in cycles."]
pub type TckehcsF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TZQCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tzqcke_f1(&self) -> TzqckeF1R {
        TzqckeF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TCSCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tcscke_f2(&self) -> TcsckeF2R {
        TcsckeF2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCS value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tckelcs_f2(&self) -> TckelcsF2R {
        TckelcsF2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCS value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tckehcs_f2(&self) -> TckehcsF2R {
        TckehcsF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TZQCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqcke_f1(&mut self) -> TzqckeF1W<DdrDenaliCtl58Spec> {
        TzqckeF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TCSCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcscke_f2(&mut self) -> TcsckeF2W<DdrDenaliCtl58Spec> {
        TcsckeF2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DRAM TCKELCS value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelcs_f2(&mut self) -> TckelcsF2W<DdrDenaliCtl58Spec> {
        TckelcsF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKEHCS value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckehcs_f2(&mut self) -> TckehcsF2W<DdrDenaliCtl58Spec> {
        TckehcsF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_58::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_58::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl58Spec;
impl crate::RegisterSpec for DdrDenaliCtl58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_58::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl58Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_58::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_58 to value 0"]
impl crate::Resettable for DdrDenaliCtl58Spec {
    const RESET_VALUE: u32 = 0;
}
