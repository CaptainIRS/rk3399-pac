#[doc = "Register `DENALI_CTL_63` reader"]
pub type R = crate::R<DenaliCtl63Spec>;
#[doc = "Register `DENALI_CTL_63` writer"]
pub type W = crate::W<DenaliCtl63Spec>;
#[doc = "Field `TCKCKEL_F0` reader - DRAM TCKCKEL value for frequency copy 0 in cycles."]
pub type TckckelF0R = crate::FieldReader;
#[doc = "Field `TCKCKEL_F0` writer - DRAM TCKCKEL value for frequency copy 0 in cycles."]
pub type TckckelF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSR_F0` reader - DRAM TSR value for frequency copy 0 in cycles."]
pub type TsrF0R = crate::FieldReader;
#[doc = "Field `TSR_F0` writer - DRAM TSR value for frequency copy 0 in cycles."]
pub type TsrF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TESCKE_F0` reader - DRAM TESCKE value for frequency copy 0 in cycles."]
pub type TesckeF0R = crate::FieldReader;
#[doc = "Field `TESCKE_F0` writer - DRAM TESCKE value for frequency copy 0 in cycles."]
pub type TesckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCKELPD_F0` reader - DRAM TCKELPD value for frequency copy 0 in cycles."]
pub type TckelpdF0R = crate::FieldReader;
#[doc = "Field `TCKELPD_F0` writer - DRAM TCKELPD value for frequency copy 0 in cycles."]
pub type TckelpdF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCKCKEL value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tckckel_f0(&self) -> TckckelF0R {
        TckckelF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TSR value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tsr_f0(&self) -> TsrF0R {
        TsrF0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - DRAM TESCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tescke_f0(&self) -> TesckeF0R {
        TesckeF0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKELPD value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tckelpd_f0(&self) -> TckelpdF0R {
        TckelpdF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCKCKEL value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckckel_f0(&mut self) -> TckckelF0W<DenaliCtl63Spec> {
        TckckelF0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TSR value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tsr_f0(&mut self) -> TsrF0W<DenaliCtl63Spec> {
        TsrF0W::new(self, 8)
    }
    #[doc = "Bits 16:18 - DRAM TESCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tescke_f0(&mut self) -> TesckeF0W<DenaliCtl63Spec> {
        TesckeF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKELPD value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelpd_f0(&mut self) -> TckelpdF0W<DenaliCtl63Spec> {
        TckelpdF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl63Spec;
impl crate::RegisterSpec for DenaliCtl63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_63::R`](R) reader structure"]
impl crate::Readable for DenaliCtl63Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_63::W`](W) writer structure"]
impl crate::Writable for DenaliCtl63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_63 to value 0"]
impl crate::Resettable for DenaliCtl63Spec {
    const RESET_VALUE: u32 = 0;
}
