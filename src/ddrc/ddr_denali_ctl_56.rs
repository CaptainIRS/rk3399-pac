#[doc = "Register `DDR_DENALI_CTL_56` reader"]
pub type R = crate::R<DdrDenaliCtl56Spec>;
#[doc = "Register `DDR_DENALI_CTL_56` writer"]
pub type W = crate::W<DdrDenaliCtl56Spec>;
#[doc = "Field `TCKELCS_F0` reader - DRAM TCKELCS value for frequency copy 0 in cycles."]
pub type TckelcsF0R = crate::FieldReader;
#[doc = "Field `TCKELCS_F0` writer - DRAM TCKELCS value for frequency copy 0 in cycles."]
pub type TckelcsF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TCKEHCS_F0` reader - DRAM TCKEHCS value for frequency copy 0 in cycles."]
pub type TckehcsF0R = crate::FieldReader;
#[doc = "Field `TCKEHCS_F0` writer - DRAM TCKEHCS value for frequency copy 0 in cycles."]
pub type TckehcsF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TMRWCKEL_F0` reader - DRAM TMRWCKEL value for frequency copy 0 in cycles."]
pub type TmrwckelF0R = crate::FieldReader;
#[doc = "Field `TMRWCKEL_F0` writer - DRAM TMRWCKEL value for frequency copy 0 in cycles."]
pub type TmrwckelF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TZQCKE_F0` reader - DRAM TZQCKE value for frequency copy 0 in cycles."]
pub type TzqckeF0R = crate::FieldReader;
#[doc = "Field `TZQCKE_F0` writer - DRAM TZQCKE value for frequency copy 0 in cycles."]
pub type TzqckeF0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DRAM TCKELCS value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tckelcs_f0(&self) -> TckelcsF0R {
        TckelcsF0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DRAM TCKEHCS value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tckehcs_f0(&self) -> TckehcsF0R {
        TckehcsF0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - DRAM TMRWCKEL value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tmrwckel_f0(&self) -> TmrwckelF0R {
        TmrwckelF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - DRAM TZQCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tzqcke_f0(&self) -> TzqckeF0R {
        TzqckeF0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DRAM TCKELCS value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckelcs_f0(&mut self) -> TckelcsF0W<DdrDenaliCtl56Spec> {
        TckelcsF0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - DRAM TCKEHCS value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tckehcs_f0(&mut self) -> TckehcsF0W<DdrDenaliCtl56Spec> {
        TckehcsF0W::new(self, 8)
    }
    #[doc = "Bits 16:20 - DRAM TMRWCKEL value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrwckel_f0(&mut self) -> TmrwckelF0W<DdrDenaliCtl56Spec> {
        TmrwckelF0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - DRAM TZQCKE value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqcke_f0(&mut self) -> TzqckeF0W<DdrDenaliCtl56Spec> {
        TzqckeF0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_56::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_56::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl56Spec;
impl crate::RegisterSpec for DdrDenaliCtl56Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_56::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl56Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_56::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_56 to value 0"]
impl crate::Resettable for DdrDenaliCtl56Spec {
    const RESET_VALUE: u32 = 0;
}
