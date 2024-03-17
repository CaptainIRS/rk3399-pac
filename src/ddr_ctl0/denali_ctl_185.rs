#[doc = "Register `DENALI_CTL_185` reader"]
pub type R = crate::R<DenaliCtl185Spec>;
#[doc = "Register `DENALI_CTL_185` writer"]
pub type W = crate::W<DenaliCtl185Spec>;
#[doc = "Field `ZQCL_F2` reader - Number of cycles needed for a ZQCL command for frequency copy 2."]
pub type ZqclF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQCL_F2` writer - Number of cycles needed for a ZQCL command for frequency copy 2."]
pub type ZqclF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `ZQCS_F2` reader - Number of cycles needed for a ZQCS command for frequency copy 2."]
pub type ZqcsF2R = crate::FieldReader<u16>;
#[doc = "Field `ZQCS_F2` writer - Number of cycles needed for a ZQCS command for frequency copy 2."]
pub type ZqcsF2W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQCL command for frequency copy 2."]
    #[inline(always)]
    pub fn zqcl_f2(&self) -> ZqclF2R {
        ZqclF2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQCS command for frequency copy 2."]
    #[inline(always)]
    pub fn zqcs_f2(&self) -> ZqcsF2R {
        ZqcsF2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of cycles needed for a ZQCL command for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zqcl_f2(&mut self) -> ZqclF2W<DenaliCtl185Spec> {
        ZqclF2W::new(self, 0)
    }
    #[doc = "Bits 16:27 - Number of cycles needed for a ZQCS command for frequency copy 2."]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_f2(&mut self) -> ZqcsF2W<DenaliCtl185Spec> {
        ZqcsF2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_185::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_185::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl185Spec;
impl crate::RegisterSpec for DenaliCtl185Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_185::R`](R) reader structure"]
impl crate::Readable for DenaliCtl185Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_185::W`](W) writer structure"]
impl crate::Writable for DenaliCtl185Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_185 to value 0"]
impl crate::Resettable for DenaliCtl185Spec {
    const RESET_VALUE: u32 = 0;
}
