#[doc = "Register `CP_GR_G` reader"]
pub type R = crate::R<CpGrGSpec>;
#[doc = "Register `CP_GR_G` writer"]
pub type W = crate::W<CpGrGSpec>;
#[doc = "Field `SW_GRADIENT_X_G` reader - X gradient value of Green (signed 8.8)"]
pub type SwGradientXGR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_X_G` writer - X gradient value of Green (signed 8.8)"]
pub type SwGradientXGW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_GRADIENT_Y_G` reader - Y gradient value of Green (signed 8.8)"]
pub type SwGradientYGR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_Y_G` writer - Y gradient value of Green (signed 8.8)"]
pub type SwGradientYGW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - X gradient value of Green (signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_x_g(&self) -> SwGradientXGR {
        SwGradientXGR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Y gradient value of Green (signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_y_g(&self) -> SwGradientYGR {
        SwGradientYGR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X gradient value of Green (signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_x_g(&mut self) -> SwGradientXGW<CpGrGSpec> {
        SwGradientXGW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y gradient value of Green (signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_y_g(&mut self) -> SwGradientYGW<CpGrGSpec> {
        SwGradientYGW::new(self, 16)
    }
}
#[doc = "RGA color gradient fill step register (color fill mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpGrGSpec;
impl crate::RegisterSpec for CpGrGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cp_gr_g::R`](R) reader structure"]
impl crate::Readable for CpGrGSpec {}
#[doc = "`write(|w| ..)` method takes [`cp_gr_g::W`](W) writer structure"]
impl crate::Writable for CpGrGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CP_GR_G to value 0"]
impl crate::Resettable for CpGrGSpec {
    const RESET_VALUE: u32 = 0;
}
