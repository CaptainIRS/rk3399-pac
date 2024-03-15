#[doc = "Register `DENALI_CTL_67` reader"]
pub type R = crate::R<DenaliCtl67Spec>;
#[doc = "Register `DENALI_CTL_67` writer"]
pub type W = crate::W<DenaliCtl67Spec>;
#[doc = "Field `TCKCKEL_F2` reader - DRAM TCKCKEL value for frequency copy 2 in cycles."]
pub type TckckelF2R = crate::FieldReader;
#[doc = "Field `TCKCKEL_F2` writer - DRAM TCKCKEL value for frequency copy 2 in cycles."]
pub type TckckelF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSR_F2` reader - DRAM TSR value for frequency copy 2 in cycles."]
pub type TsrF2R = crate::FieldReader;
#[doc = "Field `TSR_F2` writer - DRAM TSR value for frequency copy 2 in cycles."]
pub type TsrF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TESCKE_F2` reader - DRAM TESCKE value for frequency copy 2 in cycles."]
pub type TesckeF2R = crate::FieldReader;
#[doc = "Field `TESCKE_F2` writer - DRAM TESCKE value for frequency copy 2 in cycles."]
pub type TesckeF2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCKELPD_F2` reader - DRAM TCKELPD value for frequency copy 2 in cycles."]
pub type TckelpdF2R = crate::FieldReader;
#[doc = "Field `TCKELPD_F2` writer - DRAM TCKELPD value for frequency copy 2 in cycles."]
pub type TckelpdF2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCKCKEL value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tckckel_f2(&self) -> TckckelF2R {
        TckckelF2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - DRAM TSR value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tsr_f2(&self) -> TsrF2R {
        TsrF2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - DRAM TESCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tescke_f2(&self) -> TesckeF2R {
        TesckeF2R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TCKELPD value for frequency copy 2 in cycles."]
    #[inline(always)]
    pub fn tckelpd_f2(&self) -> TckelpdF2R {
        TckelpdF2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCKCKEL value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckckel_f2(&mut self) -> TckckelF2W<DenaliCtl67Spec> {
        TckckelF2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DRAM TSR value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tsr_f2(&mut self) -> TsrF2W<DenaliCtl67Spec> {
        TsrF2W::new(self, 8)
    }
    #[doc = "Bits 16:18 - DRAM TESCKE value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tescke_f2(&mut self) -> TesckeF2W<DenaliCtl67Spec> {
        TesckeF2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TCKELPD value for frequency copy 2 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelpd_f2(&mut self) -> TckelpdF2W<DenaliCtl67Spec> {
        TckelpdF2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_67::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_67::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl67Spec;
impl crate::RegisterSpec for DenaliCtl67Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_67::R`](R) reader structure"]
impl crate::Readable for DenaliCtl67Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_67::W`](W) writer structure"]
impl crate::Writable for DenaliCtl67Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_67 to value 0"]
impl crate::Resettable for DenaliCtl67Spec {
    const RESET_VALUE: u32 = 0;
}
