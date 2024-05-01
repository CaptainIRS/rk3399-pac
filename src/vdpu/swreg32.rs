#[doc = "Register `SWREG32` reader"]
pub type R = crate::R<Swreg32Spec>;
#[doc = "Register `SWREG32` writer"]
pub type W = crate::W<Swreg32Spec>;
#[doc = "Field `SW_CONT_OFFSET0` reader - the offset value 0 for contrast adjust"]
pub type SwContOffset0R = crate::FieldReader<u16>;
#[doc = "Field `SW_CONT_OFFSET0` writer - the offset value 0 for contrast adjust"]
pub type SwContOffset0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_CONT_OFFSET1` reader - the offset value 1 for contrast adjust"]
pub type SwContOffset1R = crate::FieldReader<u16>;
#[doc = "Field `SW_CONT_OFFSET1` writer - the offset value 1 for contrast adjust"]
pub type SwContOffset1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - the offset value 0 for contrast adjust"]
    #[inline(always)]
    pub fn sw_cont_offset0(&self) -> SwContOffset0R {
        SwContOffset0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - the offset value 1 for contrast adjust"]
    #[inline(always)]
    pub fn sw_cont_offset1(&self) -> SwContOffset1R {
        SwContOffset1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - the offset value 0 for contrast adjust"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cont_offset0(&mut self) -> SwContOffset0W<Swreg32Spec> {
        SwContOffset0W::new(self, 0)
    }
    #[doc = "Bits 16:25 - the offset value 1 for contrast adjust"]
    #[inline(always)]
    #[must_use]
    pub fn sw_cont_offset1(&mut self) -> SwContOffset1W<Swreg32Spec> {
        SwContOffset1W::new(self, 16)
    }
}
#[doc = "contrast adjust offset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg32Spec;
impl crate::RegisterSpec for Swreg32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg32::R`](R) reader structure"]
impl crate::Readable for Swreg32Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg32::W`](W) writer structure"]
impl crate::Writable for Swreg32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG32 to value 0"]
impl crate::Resettable for Swreg32Spec {
    const RESET_VALUE: u32 = 0;
}
