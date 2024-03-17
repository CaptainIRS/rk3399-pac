#[doc = "Register `DENALI_CTL_57` reader"]
pub type R = crate::R<DenaliCtl57Spec>;
#[doc = "Register `DENALI_CTL_57` writer"]
pub type W = crate::W<DenaliCtl57Spec>;
#[doc = "Field `TCSCKE_F1` reader - DRAM TCSCKE value for frequency copy 1 in cycles."]
pub type TcsckeF1R = crate::FieldReader;
#[doc = "Field `TCSCKE_F1` writer - DRAM TCSCKE value for frequency copy 1 in cycles."]
pub type TcsckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKELCS_F1` reader - DRAM TCKELCS value for frequency copy 1 in cycles."]
pub type TckelcsF1R = crate::FieldReader;
#[doc = "Field `TCKELCS_F1` writer - DRAM TCKELCS value for frequency copy 1 in cycles."]
pub type TckelcsF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKEHCS_F1` reader - DRAM TCKEHCS value for frequency copy 1 in cycles."]
pub type TckehcsF1R = crate::FieldReader;
#[doc = "Field `TCKEHCS_F1` writer - DRAM TCKEHCS value for frequency copy 1 in cycles."]
pub type TckehcsF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMRWCKEL_F1` reader - DRAM TMRWCKEL value for frequency copy 1 in cycles."]
pub type TmrwckelF1R = crate::FieldReader;
#[doc = "Field `TMRWCKEL_F1` writer - DRAM TMRWCKEL value for frequency copy 1 in cycles."]
pub type TmrwckelF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCSCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tcscke_f1(&self) -> TcsckeF1R {
        TcsckeF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TCKELCS value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tckelcs_f1(&self) -> TckelcsF1R {
        TckelcsF1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DRAM TCKEHCS value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tckehcs_f1(&self) -> TckehcsF1R {
        TckehcsF1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - DRAM TMRWCKEL value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tmrwckel_f1(&self) -> TmrwckelF1R {
        TmrwckelF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCSCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcscke_f1(&mut self) -> TcsckeF1W<DenaliCtl57Spec> {
        TcsckeF1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TCKELCS value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelcs_f1(&mut self) -> TckelcsF1W<DenaliCtl57Spec> {
        TckelcsF1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - DRAM TCKEHCS value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckehcs_f1(&mut self) -> TckehcsF1W<DenaliCtl57Spec> {
        TckehcsF1W::new(self, 16)
    }
    #[doc = "Bits 24:28 - DRAM TMRWCKEL value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrwckel_f1(&mut self) -> TmrwckelF1W<DenaliCtl57Spec> {
        TmrwckelF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_57::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_57::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl57Spec;
impl crate::RegisterSpec for DenaliCtl57Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_57::R`](R) reader structure"]
impl crate::Readable for DenaliCtl57Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_57::W`](W) writer structure"]
impl crate::Writable for DenaliCtl57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_57 to value 0"]
impl crate::Resettable for DenaliCtl57Spec {
    const RESET_VALUE: u32 = 0;
}
