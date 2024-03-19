#[doc = "Register `DDR_DENALI_CTL_42` reader"]
pub type R = crate::R<DdrDenaliCtl42Spec>;
#[doc = "Register `DDR_DENALI_CTL_42` writer"]
pub type W = crate::W<DdrDenaliCtl42Spec>;
#[doc = "Field `TCAEXT` reader - DRAM TCAEXT value in cycles."]
pub type TcaextR = crate::FieldReader;
#[doc = "Field `TCAEXT` writer - DRAM TCAEXT value in cycles."]
pub type TcaextW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TCACKEH` reader - DRAM TCACKEH value in cycles."]
pub type TcackehR = crate::FieldReader;
#[doc = "Field `TCACKEH` writer - DRAM TCACKEH value in cycles."]
pub type TcackehW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TMRZ_F0` reader - DRAM TMRZ value for frequency copy 0 in cycles."]
pub type TmrzF0R = crate::FieldReader;
#[doc = "Field `TMRZ_F0` writer - DRAM TMRZ value for frequency copy 0 in cycles."]
pub type TmrzF0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TMRZ_F1` reader - DRAM TMRZ value for frequency copy 1 in cycles."]
pub type TmrzF1R = crate::FieldReader;
#[doc = "Field `TMRZ_F1` writer - DRAM TMRZ value for frequency copy 1 in cycles."]
pub type TmrzF1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DRAM TCAEXT value in cycles."]
    #[inline(always)]
    pub fn tcaext(&self) -> TcaextR {
        TcaextR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DRAM TCACKEH value in cycles."]
    #[inline(always)]
    pub fn tcackeh(&self) -> TcackehR {
        TcackehR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DRAM TMRZ value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tmrz_f0(&self) -> TmrzF0R {
        TmrzF0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - DRAM TMRZ value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tmrz_f1(&self) -> TmrzF1R {
        TmrzF1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DRAM TCAEXT value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcaext(&mut self) -> TcaextW<DdrDenaliCtl42Spec> {
        TcaextW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DRAM TCACKEH value in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tcackeh(&mut self) -> TcackehW<DdrDenaliCtl42Spec> {
        TcackehW::new(self, 8)
    }
    #[doc = "Bits 16:20 - DRAM TMRZ value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrz_f0(&mut self) -> TmrzF0W<DdrDenaliCtl42Spec> {
        TmrzF0W::new(self, 16)
    }
    #[doc = "Bits 24:28 - DRAM TMRZ value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tmrz_f1(&mut self) -> TmrzF1W<DdrDenaliCtl42Spec> {
        TmrzF1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_42::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_42::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl42Spec;
impl crate::RegisterSpec for DdrDenaliCtl42Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_42::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl42Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_42::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_42 to value 0"]
impl crate::Resettable for DdrDenaliCtl42Spec {
    const RESET_VALUE: u32 = 0;
}
