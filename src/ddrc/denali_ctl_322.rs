#[doc = "Register `DENALI_CTL_322` reader"]
pub type R = crate::R<DenaliCtl322Spec>;
#[doc = "Register `DENALI_CTL_322` writer"]
pub type W = crate::W<DenaliCtl322Spec>;
#[doc = "Field `USER_DEF_REG_COPIED_F2_0` reader - User-defined copied output register 0."]
pub type UserDefRegCopiedF2_0R = crate::FieldReader<u32>;
#[doc = "Field `USER_DEF_REG_COPIED_F2_0` writer - User-defined copied output register 0."]
pub type UserDefRegCopiedF2_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined copied output register 0."]
    #[inline(always)]
    pub fn user_def_reg_copied_f2_0(&self) -> UserDefRegCopiedF2_0R {
        UserDefRegCopiedF2_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined copied output register 0."]
    #[inline(always)]
    #[must_use]
    pub fn user_def_reg_copied_f2_0(&mut self) -> UserDefRegCopiedF2_0W<DenaliCtl322Spec> {
        UserDefRegCopiedF2_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_322::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_322::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl322Spec;
impl crate::RegisterSpec for DenaliCtl322Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_322::R`](R) reader structure"]
impl crate::Readable for DenaliCtl322Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_322::W`](W) writer structure"]
impl crate::Writable for DenaliCtl322Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_322 to value 0"]
impl crate::Resettable for DenaliCtl322Spec {
    const RESET_VALUE: u32 = 0;
}
