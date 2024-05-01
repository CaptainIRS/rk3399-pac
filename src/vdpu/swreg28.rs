#[doc = "Register `SWREG28` reader"]
pub type R = crate::R<Swreg28Spec>;
#[doc = "Register `SWREG28` writer"]
pub type W = crate::W<Swreg28Spec>;
#[doc = "Field `SW_ST_COORDX_MA2` reader - the start x-coordinate of mask area 2 of Horizontal start pixel"]
pub type SwStCoordxMa2R = crate::FieldReader<u16>;
#[doc = "Field `SW_ST_COORDX_MA2` writer - the start x-coordinate of mask area 2 of Horizontal start pixel"]
pub type SwStCoordxMa2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_END_COORDX_MA2` reader - the end x-coordinate of mask area 2 of Horizontal start pixel\n\nrange:sw_st_coordx_ma2~dst width"]
pub type SwEndCoordxMa2R = crate::FieldReader<u16>;
#[doc = "Field `SW_END_COORDX_MA2` writer - the end x-coordinate of mask area 2 of Horizontal start pixel\n\nrange:sw_st_coordx_ma2~dst width"]
pub type SwEndCoordxMa2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - the start x-coordinate of mask area 2 of Horizontal start pixel"]
    #[inline(always)]
    pub fn sw_st_coordx_ma2(&self) -> SwStCoordxMa2R {
        SwStCoordxMa2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - the end x-coordinate of mask area 2 of Horizontal start pixel\n\nrange:sw_st_coordx_ma2~dst width"]
    #[inline(always)]
    pub fn sw_end_coordx_ma2(&self) -> SwEndCoordxMa2R {
        SwEndCoordxMa2R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - the start x-coordinate of mask area 2 of Horizontal start pixel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_st_coordx_ma2(&mut self) -> SwStCoordxMa2W<Swreg28Spec> {
        SwStCoordxMa2W::new(self, 0)
    }
    #[doc = "Bits 11:21 - the end x-coordinate of mask area 2 of Horizontal start pixel\n\nrange:sw_st_coordx_ma2~dst width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_end_coordx_ma2(&mut self) -> SwEndCoordxMa2W<Swreg28Spec> {
        SwEndCoordxMa2W::new(self, 11)
    }
}
#[doc = "x-coordinate of mask area 2 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg28::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg28::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg28Spec;
impl crate::RegisterSpec for Swreg28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg28::R`](R) reader structure"]
impl crate::Readable for Swreg28Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg28::W`](W) writer structure"]
impl crate::Writable for Swreg28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG28 to value 0"]
impl crate::Resettable for Swreg28Spec {
    const RESET_VALUE: u32 = 0;
}
