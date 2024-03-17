#[doc = "Register `DENALI_CTL_183` reader"]
pub type R = crate::R<DenaliCtl183Spec>;
#[doc = "Register `DENALI_CTL_183` writer"]
pub type W = crate::W<DenaliCtl183Spec>;
#[doc = "Field `ZQCS_F1` reader - Number of cycles needed for a ZQCS command for frequency copy 1."]
pub type ZqcsF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQCS_F1` writer - Number of cycles needed for a ZQCS command for frequency copy 1."]
pub type ZqcsF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TZQCAL_F1` reader - Holds the DRAM ZQCAL value for frequency copy 1 in cycles."]
pub type TzqcalF1R = crate::FieldReader<u16>;
#[doc = "Field `TZQCAL_F1` writer - Holds the DRAM ZQCAL value for frequency copy 1 in cycles."]
pub type TzqcalF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQCS command for frequency copy 1."]
    #[inline(always)]
    pub fn zqcs_f1(&self) -> ZqcsF1R {
        ZqcsF1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Holds the DRAM ZQCAL value for frequency copy 1 in cycles."]
    #[inline(always)]
    pub fn tzqcal_f1(&self) -> TzqcalF1R {
        TzqcalF1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQCS command for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_f1(&mut self) -> ZqcsF1W<DenaliCtl183Spec> {
        ZqcsF1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Holds the DRAM ZQCAL value for frequency copy 1 in cycles."]
    #[inline(always)]
    #[must_use]
    pub fn tzqcal_f1(&mut self) -> TzqcalF1W<DenaliCtl183Spec> {
        TzqcalF1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_183::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_183::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl183Spec;
impl crate::RegisterSpec for DenaliCtl183Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_183::R`](R) reader structure"]
impl crate::Readable for DenaliCtl183Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_183::W`](W) writer structure"]
impl crate::Writable for DenaliCtl183Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_183 to value 0"]
impl crate::Resettable for DenaliCtl183Spec {
    const RESET_VALUE: u32 = 0;
}
