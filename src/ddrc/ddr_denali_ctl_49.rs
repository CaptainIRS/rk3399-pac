#[doc = "Register `DDR_DENALI_CTL_49` reader"]
pub type R = crate::R<DdrDenaliCtl49Spec>;
#[doc = "Register `DDR_DENALI_CTL_49` writer"]
pub type W = crate::W<DdrDenaliCtl49Spec>;
#[doc = "Field `TRFC_F1` reader - DRAM TRFC value for frequency copy 1 in cycles."]
pub type TrfcF1R = crate::FieldReader<u16>;
#[doc = "Field `TRFC_F1` writer - DRAM TRFC value for frequency copy 1 in cycles."]
pub type TrfcF1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TREF_F1` reader - DRAM TREF value for frequency copy 1 in cycles."]
pub type TrefF1R = crate::FieldReader<u16>;
#[doc = "Field `TREF_F1` writer - DRAM TREF value for frequency copy 1 in cycles."]
pub type TrefF1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - DRAM TRFC value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn trfc_f1(&self) -> TrfcF1R {
        TrfcF1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - DRAM TREF value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tref_f1(&self) -> TrefF1R {
        TrefF1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DRAM TRFC value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn trfc_f1(&mut self) -> TrfcF1W<DdrDenaliCtl49Spec> {
        TrfcF1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - DRAM TREF value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tref_f1(&mut self) -> TrefF1W<DdrDenaliCtl49Spec> {
        TrefF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl49Spec;
impl crate::RegisterSpec for DdrDenaliCtl49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_49::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl49Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_49::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_49 to value 0"]
impl crate::Resettable for DdrDenaliCtl49Spec {
    const RESET_VALUE: u32 = 0;
}
