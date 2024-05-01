#[doc = "Register `SWREG5` reader"]
pub type R = crate::R<Swreg5Spec>;
#[doc = "Register `SWREG5` writer"]
pub type W = crate::W<Swreg5Spec>;
#[doc = "Field `SW_SCL_FCT_H` reader - scaling factor of height\n\nvalue = (output_width-1)/(input_width-1)"]
pub type SwSclFctHR = crate::FieldReader<u32>;
#[doc = "Field `SW_SCL_FCT_H` writer - scaling factor of height\n\nvalue = (output_width-1)/(input_width-1)"]
pub type SwSclFctHW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:17 - scaling factor of height\n\nvalue = (output_width-1)/(input_width-1)"]
    #[inline(always)]
    pub fn sw_scl_fct_h(&self) -> SwSclFctHR {
        SwSclFctHR::new(self.bits & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:17 - scaling factor of height\n\nvalue = (output_width-1)/(input_width-1)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scl_fct_h(&mut self) -> SwSclFctHW<Swreg5Spec> {
        SwSclFctHW::new(self, 0)
    }
}
#[doc = "scl ctrl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg5Spec;
impl crate::RegisterSpec for Swreg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg5::R`](R) reader structure"]
impl crate::Readable for Swreg5Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg5::W`](W) writer structure"]
impl crate::Writable for Swreg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG5 to value 0"]
impl crate::Resettable for Swreg5Spec {
    const RESET_VALUE: u32 = 0;
}
