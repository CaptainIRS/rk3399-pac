#[doc = "Register `SWREG29` reader"]
pub type R = crate::R<Swreg29Spec>;
#[doc = "Register `SWREG29` writer"]
pub type W = crate::W<Swreg29Spec>;
#[doc = "Field `SW_ST_COORDY_MA2` reader - the start y-coordinate of mask area 2 of Vertical start pixel"]
pub type SwStCoordyMa2R = crate::FieldReader<u16>;
#[doc = "Field `SW_ST_COORDY_MA2` writer - the start y-coordinate of mask area 2 of Vertical start pixel"]
pub type SwStCoordyMa2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_END_COORDY_MA2` reader - the start y-coordinate of mask area 2 of Vertical start pixel\n\nrange:sw_st_coordy_ma2-dst width"]
pub type SwEndCoordyMa2R = crate::FieldReader<u16>;
#[doc = "Field `SW_END_COORDY_MA2` writer - the start y-coordinate of mask area 2 of Vertical start pixel\n\nrange:sw_st_coordy_ma2-dst width"]
pub type SwEndCoordyMa2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - the start y-coordinate of mask area 2 of Vertical start pixel"]
    #[inline(always)]
    pub fn sw_st_coordy_ma2(&self) -> SwStCoordyMa2R {
        SwStCoordyMa2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - the start y-coordinate of mask area 2 of Vertical start pixel\n\nrange:sw_st_coordy_ma2-dst width"]
    #[inline(always)]
    pub fn sw_end_coordy_ma2(&self) -> SwEndCoordyMa2R {
        SwEndCoordyMa2R::new(((self.bits >> 11) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - the start y-coordinate of mask area 2 of Vertical start pixel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_st_coordy_ma2(&mut self) -> SwStCoordyMa2W<Swreg29Spec> {
        SwStCoordyMa2W::new(self, 0)
    }
    #[doc = "Bits 11:21 - the start y-coordinate of mask area 2 of Vertical start pixel\n\nrange:sw_st_coordy_ma2-dst width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_end_coordy_ma2(&mut self) -> SwEndCoordyMa2W<Swreg29Spec> {
        SwEndCoordyMa2W::new(self, 11)
    }
}
#[doc = "y-coordinate of mask area 2 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg29Spec;
impl crate::RegisterSpec for Swreg29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg29::R`](R) reader structure"]
impl crate::Readable for Swreg29Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg29::W`](W) writer structure"]
impl crate::Writable for Swreg29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG29 to value 0"]
impl crate::Resettable for Swreg29Spec {
    const RESET_VALUE: u32 = 0;
}
