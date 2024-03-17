#[doc = "Register `DENALI_CTL_321` reader"]
pub type R = crate::R<DenaliCtl321Spec>;
#[doc = "Register `DENALI_CTL_321` writer"]
pub type W = crate::W<DenaliCtl321Spec>;
#[doc = "Field `USER_DEF_REG_COPIED_F1_1` reader - User-defined copied output register 1."]
pub type UserDefRegCopiedF1_1R = crate::FieldReader<u32>;
#[doc = "Field `USER_DEF_REG_COPIED_F1_1` writer - User-defined copied output register 1."]
pub type UserDefRegCopiedF1_1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - User-defined copied output register 1."]
    #[inline(always)]
    pub fn user_def_reg_copied_f1_1(&self) -> UserDefRegCopiedF1_1R {
        UserDefRegCopiedF1_1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - User-defined copied output register 1."]
    #[inline(always)]
    #[must_use]
    pub fn user_def_reg_copied_f1_1(&mut self) -> UserDefRegCopiedF1_1W<DenaliCtl321Spec> {
        UserDefRegCopiedF1_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_321::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_321::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl321Spec;
impl crate::RegisterSpec for DenaliCtl321Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_321::R`](R) reader structure"]
impl crate::Readable for DenaliCtl321Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_321::W`](W) writer structure"]
impl crate::Writable for DenaliCtl321Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_321 to value 0"]
impl crate::Resettable for DenaliCtl321Spec {
    const RESET_VALUE: u32 = 0;
}
