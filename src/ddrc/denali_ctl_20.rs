#[doc = "Register `DENALI_CTL_20` reader"]
pub type R = crate::R<DenaliCtl20Spec>;
#[doc = "Register `DENALI_CTL_20` writer"]
pub type W = crate::W<DenaliCtl20Spec>;
#[doc = "Field `TRST_PWRON` reader - Duration of memory reset during power-on initialization."]
pub type TrstPwronR = crate::FieldReader<u32>;
#[doc = "Field `TRST_PWRON` writer - Duration of memory reset during power-on initialization."]
pub type TrstPwronW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Duration of memory reset during power-on initialization."]
    #[inline(always)]
    pub fn trst_pwron(&self) -> TrstPwronR {
        TrstPwronR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Duration of memory reset during power-on initialization."]
    #[inline(always)]
    #[must_use]
    pub fn trst_pwron(&mut self) -> TrstPwronW<DenaliCtl20Spec> {
        TrstPwronW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl20Spec;
impl crate::RegisterSpec for DenaliCtl20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_20::R`](R) reader structure"]
impl crate::Readable for DenaliCtl20Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_20::W`](W) writer structure"]
impl crate::Writable for DenaliCtl20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_20 to value 0"]
impl crate::Resettable for DenaliCtl20Spec {
    const RESET_VALUE: u32 = 0;
}
