#[doc = "Register `DDR_DENALI_CTL_181` reader"]
pub type R = crate::R<DdrDenaliCtl181Spec>;
#[doc = "Register `DDR_DENALI_CTL_181` writer"]
pub type W = crate::W<DdrDenaliCtl181Spec>;
#[doc = "Field `TZQCAL_F0` reader - Holds the DRAM ZQCAL value for frequency copy 0 in cycles."]
pub type TzqcalF0R = crate::FieldReader<u16>;
#[doc = "Field `TZQCAL_F0` writer - Holds the DRAM ZQCAL value for frequency copy 0 in cycles."]
pub type TzqcalF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TZQLAT_F0` reader - Holds the DRAM ZQLAT value for frequency copy 0 in cycles."]
pub type TzqlatF0R = crate::FieldReader;
#[doc = "Field `TZQLAT_F0` writer - Holds the DRAM ZQLAT value for frequency copy 0 in cycles."]
pub type TzqlatF0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:11 - Holds the DRAM ZQCAL value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tzqcal_f0(&self) -> TzqcalF0R {
        TzqcalF0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:21 - Holds the DRAM ZQLAT value for frequency copy 0 in cycles."]
    #[inline(always)]
    pub fn tzqlat_f0(&self) -> TzqlatF0R {
        TzqlatF0R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Holds the DRAM ZQCAL value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqcal_f0(&mut self) -> TzqcalF0W<DdrDenaliCtl181Spec> {
        TzqcalF0W::new(self, 0)
    }
    #[doc = "Bits 16:21 - Holds the DRAM ZQLAT value for frequency copy 0 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqlat_f0(&mut self) -> TzqlatF0W<DdrDenaliCtl181Spec> {
        TzqlatF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_181::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_181::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl181Spec;
impl crate::RegisterSpec for DdrDenaliCtl181Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_181::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl181Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_181::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl181Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_181 to value 0"]
impl crate::Resettable for DdrDenaliCtl181Spec {
    const RESET_VALUE: u32 = 0;
}
