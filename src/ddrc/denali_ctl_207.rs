#[doc = "Register `DENALI_CTL_207` reader"]
pub type R = crate::R<DenaliCtl207Spec>;
#[doc = "Register `DENALI_CTL_207` writer"]
pub type W = crate::W<DenaliCtl207Spec>;
#[doc = "Field `INT_MASK` reader - Mask for the controller_int signal from the INT_STATUS parameter."]
pub type IntMaskR = crate::FieldReader<u32>;
#[doc = "Field `INT_MASK` writer - Mask for the controller_int signal from the INT_STATUS parameter."]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask for the controller_int signal from the INT_STATUS parameter."]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask for the controller_int signal from the INT_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn int_mask(&mut self) -> IntMaskW<DenaliCtl207Spec> {
        IntMaskW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_207::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_207::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl207Spec;
impl crate::RegisterSpec for DenaliCtl207Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_207::R`](R) reader structure"]
impl crate::Readable for DenaliCtl207Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_207::W`](W) writer structure"]
impl crate::Writable for DenaliCtl207Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_207 to value 0"]
impl crate::Resettable for DenaliCtl207Spec {
    const RESET_VALUE: u32 = 0;
}
