#[doc = "Register `CP_GR_B` reader"]
pub type R = crate::R<CpGrBSpec>;
#[doc = "Register `CP_GR_B` writer"]
pub type W = crate::W<CpGrBSpec>;
#[doc = "Field `SW_GRADIENT_X_B` reader - X gradient value of Blue (signed 8.8)"]
pub type SwGradientXBR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_X_B` writer - X gradient value of Blue (signed 8.8)"]
pub type SwGradientXBW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SW_GRADIENT_Y_B` reader - Y gradient value of Blue (signed 8.8)"]
pub type SwGradientYBR = crate::FieldReader<u16>;
#[doc = "Field `SW_GRADIENT_Y_B` writer - Y gradient value of Blue (signed 8.8)"]
pub type SwGradientYBW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - X gradient value of Blue (signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_x_b(&self) -> SwGradientXBR {
        SwGradientXBR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Y gradient value of Blue (signed 8.8)"]
    #[inline(always)]
    pub fn sw_gradient_y_b(&self) -> SwGradientYBR {
        SwGradientYBR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - X gradient value of Blue (signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_x_b(&mut self) -> SwGradientXBW<CpGrBSpec> {
        SwGradientXBW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Y gradient value of Blue (signed 8.8)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_gradient_y_b(&mut self) -> SwGradientYBW<CpGrBSpec> {
        SwGradientYBW::new(self, 16)
    }
}
#[doc = "RGA source image transparency color max value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cp_gr_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cp_gr_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpGrBSpec;
impl crate::RegisterSpec for CpGrBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cp_gr_b::R`](R) reader structure"]
impl crate::Readable for CpGrBSpec {}
#[doc = "`write(|w| ..)` method takes [`cp_gr_b::W`](W) writer structure"]
impl crate::Writable for CpGrBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CP_GR_B to value 0"]
impl crate::Resettable for CpGrBSpec {
    const RESET_VALUE: u32 = 0;
}
