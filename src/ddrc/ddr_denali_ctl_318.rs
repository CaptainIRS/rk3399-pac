#[doc = "Register `DDR_DENALI_CTL_318` reader"]
pub type R = crate::R<DdrDenaliCtl318Spec>;
#[doc = "Register `DDR_DENALI_CTL_318` writer"]
pub type W = crate::W<DdrDenaliCtl318Spec>;
#[doc = "Field `USER_DEF_REG_COPIED_F0_0` reader - User-defined copied output register 0."]
pub type UserDefRegCopiedF0_0R = crate::FieldReader<u32>;
#[doc = "Field `USER_DEF_REG_COPIED_F0_0` writer - User-defined copied output register 0."]
pub type UserDefRegCopiedF0_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined copied output register 0."]
    #[inline(always)]
    pub fn user_def_reg_copied_f0_0(&self) -> UserDefRegCopiedF0_0R {
        UserDefRegCopiedF0_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined copied output register 0."]
    #[inline(always)]
    #[must_use]
    pub fn user_def_reg_copied_f0_0(&mut self) -> UserDefRegCopiedF0_0W<DdrDenaliCtl318Spec> {
        UserDefRegCopiedF0_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_318::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_318::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl318Spec;
impl crate::RegisterSpec for DdrDenaliCtl318Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_318::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl318Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_318::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl318Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_318 to value 0"]
impl crate::Resettable for DdrDenaliCtl318Spec {
    const RESET_VALUE: u32 = 0;
}
