#[doc = "Register `DDR_DENALI_CTL_319` reader"]
pub type R = crate::R<DdrDenaliCtl319Spec>;
#[doc = "Register `DDR_DENALI_CTL_319` writer"]
pub type W = crate::W<DdrDenaliCtl319Spec>;
#[doc = "Field `USER_DEF_REG_COPIED_F0_1` reader - User-defined copied output register 1."]
pub type UserDefRegCopiedF0_1R = crate::FieldReader<u32>;
#[doc = "Field `USER_DEF_REG_COPIED_F0_1` writer - User-defined copied output register 1."]
pub type UserDefRegCopiedF0_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined copied output register 1."]
    #[inline(always)]
    pub fn user_def_reg_copied_f0_1(&self) -> UserDefRegCopiedF0_1R {
        UserDefRegCopiedF0_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined copied output register 1."]
    #[inline(always)]
    #[must_use]
    pub fn user_def_reg_copied_f0_1(&mut self) -> UserDefRegCopiedF0_1W<DdrDenaliCtl319Spec> {
        UserDefRegCopiedF0_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_319::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_319::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl319Spec;
impl crate::RegisterSpec for DdrDenaliCtl319Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_319::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl319Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_319::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl319Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_319 to value 0"]
impl crate::Resettable for DdrDenaliCtl319Spec {
    const RESET_VALUE: u32 = 0;
}
