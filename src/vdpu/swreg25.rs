#[doc = "Register `SWREG25` reader"]
pub type R = crate::R<Swreg25Spec>;
#[doc = "Register `SWREG25` writer"]
pub type W = crate::W<Swreg25Spec>;
#[doc = "Field `SW_SCANL_ABLD1` reader - ablend 1 of pixels scanline\n\ncorresponding function should be enabled"]
pub type SwScanlAbld1R = crate::FieldReader<u16>;
#[doc = "Field `SW_SCANL_ABLD1` writer - ablend 1 of pixels scanline\n\ncorresponding function should be enabled"]
pub type SwScanlAbld1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `SW_SCANL_ABLD2` reader - ablend 2 of pixels scanline\n\ncorresponding function should be enabled"]
pub type SwScanlAbld2R = crate::FieldReader<u16>;
#[doc = "Field `SW_SCANL_ABLD2` writer - ablend 2 of pixels scanline\n\ncorresponding function should be enabled"]
pub type SwScanlAbld2W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - ablend 1 of pixels scanline\n\ncorresponding function should be enabled"]
    #[inline(always)]
    pub fn sw_scanl_abld1(&self) -> SwScanlAbld1R {
        SwScanlAbld1R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - ablend 2 of pixels scanline\n\ncorresponding function should be enabled"]
    #[inline(always)]
    pub fn sw_scanl_abld2(&self) -> SwScanlAbld2R {
        SwScanlAbld2R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - ablend 1 of pixels scanline\n\ncorresponding function should be enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scanl_abld1(&mut self) -> SwScanlAbld1W<Swreg25Spec> {
        SwScanlAbld1W::new(self, 0)
    }
    #[doc = "Bits 16:28 - ablend 2 of pixels scanline\n\ncorresponding function should be enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scanl_abld2(&mut self) -> SwScanlAbld2W<Swreg25Spec> {
        SwScanlAbld2W::new(self, 16)
    }
}
#[doc = "ablend of pixels scanline\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg25Spec;
impl crate::RegisterSpec for Swreg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg25::R`](R) reader structure"]
impl crate::Readable for Swreg25Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg25::W`](W) writer structure"]
impl crate::Writable for Swreg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG25 to value 0"]
impl crate::Resettable for Swreg25Spec {
    const RESET_VALUE: u32 = 0;
}
