#[doc = "Register `CP_GR_R` reader"]
pub type R = crate::R<CpGrRSpec>;
#[doc = "Register `CP_GR_R` writer"]
pub type W = crate::W<CpGrRSpec>;
#[doc = "Field `SW_GRADIENT_X_R` reader - X gradient value of Red(signed 8.8)"]
pub type SwGradientXRR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_X_R` writer - X gradient value of Red(signed 8.8)"]
pub type SwGradientXRW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_GRADIENT_Y_R` reader - Y gradient value of Red(signed 8.8)"]
pub type SwGradientYRR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_Y_R` writer - Y gradient value of Red(signed 8.8)"]
pub type SwGradientYRW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - X gradient value of Red(signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_x_r(&self) -> SwGradientXRR {
        SwGradientXRR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Y gradient value of Red(signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_y_r(&self) -> SwGradientYRR {
        SwGradientYRR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X gradient value of Red(signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_x_r(&mut self) -> SwGradientXRW<CpGrRSpec> {
        SwGradientXRW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y gradient value of Red(signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_y_r(&mut self) -> SwGradientYRW<CpGrRSpec> {
        SwGradientYRW::new(self, 16)
    }
}
#[doc = "RGA color gradient fill step register (color fill mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpGrRSpec;
impl crate::RegisterSpec for CpGrRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cp_gr_r::R`](R) reader structure"]
impl crate::Readable for CpGrRSpec {}
#[doc = "`write(|w| ..)` method takes [`cp_gr_r::W`](W) writer structure"]
impl crate::Writable for CpGrRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CP_GR_R to value 0"]
impl crate::Resettable for CpGrRSpec {
    const RESET_VALUE: u32 = 0;
}
