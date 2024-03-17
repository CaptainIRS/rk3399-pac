#[doc = "Register `DENALI_CTL_65` reader"]
pub type R = crate::R<DenaliCtl65Spec>;
#[doc = "Register `DENALI_CTL_65` writer"]
pub type W = crate::W<DenaliCtl65Spec>;
#[doc = "Field `TCKCKEL_F1` reader - DRAM TCKCKEL value for frequency copy 1 in cycles."]
pub type TckckelF1R = crate::FieldReader;
#[doc = "Field `TCKCKEL_F1` writer - DRAM TCKCKEL value for frequency copy 1 in cycles."]
pub type TckckelF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSR_F1` reader - DRAM TSR value for frequency copy 1 in cycles."]
pub type TsrF1R = crate::FieldReader;
#[doc = "Field `TSR_F1` writer - DRAM TSR value for frequency copy 1 in cycles."]
pub type TsrF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TESCKE_F1` reader - DRAM TESCKE value for frequency copy 1 in cycles."]
pub type TesckeF1R = crate::FieldReader;
#[doc = "Field `TESCKE_F1` writer - DRAM TESCKE value for frequency copy 1 in cycles."]
pub type TesckeF1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCKELPD_F1` reader - DRAM TCKELPD value for frequency copy 1 in cycles."]
pub type TckelpdF1R = crate::FieldReader;
#[doc = "Field `TCKELPD_F1` writer - DRAM TCKELPD value for frequency copy 1 in cycles."]
pub type TckelpdF1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCKCKEL value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tckckel_f1(&self) -> TckckelF1R {
        TckckelF1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TSR value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tsr_f1(&self) -> TsrF1R {
        TsrF1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - DRAM TESCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tescke_f1(&self) -> TesckeF1R {
        TesckeF1R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKELPD value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tckelpd_f1(&self) -> TckelpdF1R {
        TckelpdF1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCKCKEL value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckckel_f1(&mut self) -> TckckelF1W<DenaliCtl65Spec> {
        TckckelF1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TSR value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tsr_f1(&mut self) -> TsrF1W<DenaliCtl65Spec> {
        TsrF1W::new(self, 8)
    }
    #[doc = "Bits 16:18 - DRAM TESCKE value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tescke_f1(&mut self) -> TesckeF1W<DenaliCtl65Spec> {
        TesckeF1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKELPD value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelpd_f1(&mut self) -> TckelpdF1W<DenaliCtl65Spec> {
        TckelpdF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl65Spec;
impl crate::RegisterSpec for DenaliCtl65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_65::R`](R) reader structure"]
impl crate::Readable for DenaliCtl65Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_65::W`](W) writer structure"]
impl crate::Writable for DenaliCtl65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_65 to value 0"]
impl crate::Resettable for DenaliCtl65Spec {
    const RESET_VALUE: u32 = 0;
}
