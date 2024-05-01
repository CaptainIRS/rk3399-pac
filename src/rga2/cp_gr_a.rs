#[doc = "Register `CP_GR_A` reader"]
pub type R = crate::R<CpGrASpec>;
#[doc = "Register `CP_GR_A` writer"]
pub type W = crate::W<CpGrASpec>;
#[doc = "Field `SW_GRADIENT_X_A` reader - X gradient value of Alpha (signed 8.8)"]
pub type SwGradientXAR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_X_A` writer - X gradient value of Alpha (signed 8.8)"]
pub type SwGradientXAW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_GRADIENT_Y_A` reader - Y gradient value of Alpha (signed 8.8)"]
pub type SwGradientYAR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_Y_A` writer - Y gradient value of Alpha (signed 8.8)"]
pub type SwGradientYAW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - X gradient value of Alpha (signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_x_a(&self) -> SwGradientXAR {
        SwGradientXAR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Y gradient value of Alpha (signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_y_a(&self) -> SwGradientYAR {
        SwGradientYAR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X gradient value of Alpha (signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_x_a(&mut self) -> SwGradientXAW<CpGrASpec> {
        SwGradientXAW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y gradient value of Alpha (signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_y_a(&mut self) -> SwGradientYAW<CpGrASpec> {
        SwGradientYAW::new(self, 16)
    }
}
#[doc = "RGA source image transparency color min value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_a::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_a::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpGrASpec;
impl crate::RegisterSpec for CpGrASpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cp_gr_a::R`](R) reader structure"]
impl crate::Readable for CpGrASpec {}
#[doc = "`write(|w| ..)` method takes [`cp_gr_a::W`](W) writer structure"]
impl crate::Writable for CpGrASpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CP_GR_A to value 0"]
impl crate::Resettable for CpGrASpec {
    const RESET_VALUE: u32 = 0;
}
