#[doc = "Register `DDR_DENALI_CTL_188` reader"]
pub type R = crate::R<DdrDenaliCtl188Spec>;
#[doc = "Register `DDR_DENALI_CTL_188` writer"]
pub type W = crate::W<DdrDenaliCtl188Spec>;
#[doc = "Field `ZQRESET_F1` reader - Number of cycles needed for a ZQRESET command for frequency copy 1."]
pub type ZqresetF1R = crate::FieldReader<u16>;
#[doc = "Field `ZQRESET_F1` writer - Number of cycles needed for a ZQRESET command for frequency copy 1."]
pub type ZqresetF1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ZQRESET_F2` reader - Number of cycles needed for a ZQRESET command for frequency copy 2."]
pub type ZqresetF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQRESET_F2` writer - Number of cycles needed for a ZQRESET command for frequency copy 2."]
pub type ZqresetF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQRESET command for frequency copy 1."]
    #[inline(always)]
    pub fn zqreset_f1(&self) -> ZqresetF1R {
        ZqresetF1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQRESET command for frequency copy 2."]
    #[inline(always)]
    pub fn zqreset_f2(&self) -> ZqresetF2R {
        ZqresetF2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQRESET command for frequency copy 1."]
    #[inline(always)]
    #[must_use]
    pub fn zqreset_f1(&mut self) -> ZqresetF1W<DdrDenaliCtl188Spec> {
        ZqresetF1W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQRESET command for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zqreset_f2(&mut self) -> ZqresetF2W<DdrDenaliCtl188Spec> {
        ZqresetF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_188::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_188::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl188Spec;
impl crate::RegisterSpec for DdrDenaliCtl188Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_188::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl188Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_188::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl188Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_188 to value 0"]
impl crate::Resettable for DdrDenaliCtl188Spec {
    const RESET_VALUE: u32 = 0;
}
