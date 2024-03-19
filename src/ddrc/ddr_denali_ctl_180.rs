#[doc = "Register `DDR_DENALI_CTL_180` reader"]
pub type R = crate::R<DdrDenaliCtl180Spec>;
#[doc = "Register `DDR_DENALI_CTL_180` writer"]
pub type W = crate::W<DdrDenaliCtl180Spec>;
#[doc = "Field `ZQCL_F0` reader - Number of cycles needed for a ZQCL command for frequency copy 0."]
pub type ZqclF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQCL_F0` writer - Number of cycles needed for a ZQCL command for frequency copy 0."]
pub type ZqclF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ZQCS_F0` reader - Number of cycles needed for a ZQCS command for frequency copy 0."]
pub type ZqcsF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQCS_F0` writer - Number of cycles needed for a ZQCS command for frequency copy 0."]
pub type ZqcsF0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQCL command for frequency copy 0."]
    #[inline(always)]
    pub fn zqcl_f0(&self) -> ZqclF0R {
        ZqclF0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQCS command for frequency copy 0."]
    #[inline(always)]
    pub fn zqcs_f0(&self) -> ZqcsF0R {
        ZqcsF0R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQCL command for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zqcl_f0(&mut self) -> ZqclF0W<DdrDenaliCtl180Spec> {
        ZqclF0W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQCS command for frequency copy 0."]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_f0(&mut self) -> ZqcsF0W<DdrDenaliCtl180Spec> {
        ZqcsF0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_180::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_180::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl180Spec;
impl crate::RegisterSpec for DdrDenaliCtl180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_180::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl180Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_180::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl180Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_180 to value 0"]
impl crate::Resettable for DdrDenaliCtl180Spec {
    const RESET_VALUE: u32 = 0;
}
