#[doc = "Register `SWREG27` reader"]
pub type R = crate::R<Swreg27Spec>;
#[doc = "Register `SWREG27` writer"]
pub type W = crate::W<Swreg27Spec>;
#[doc = "Field `SW_ST_COORDY_MA1` reader - the start y-coordinate of mask area 1 of Vertical start pixel"]
pub type SwStCoordyMa1R = crate::FieldReader<u16>;
#[doc = "Field `SW_ST_COORDY_MA1` writer - the start y-coordinate of mask area 1 of Vertical start pixel"]
pub type SwStCoordyMa1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_END_COORDY_MA1` reader - the start y-coordinate of mask area 1 of Vertical start pixel\n\nrange:sw_st_coordy_ma1~dst width"]
pub type SwEndCoordyMa1R = crate::FieldReader<u16>;
#[doc = "Field `SW_END_COORDY_MA1` writer - the start y-coordinate of mask area 1 of Vertical start pixel\n\nrange:sw_st_coordy_ma1~dst width"]
pub type SwEndCoordyMa1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - the start y-coordinate of mask area 1 of Vertical start pixel"]
    #[inline(always)]
    pub fn sw_st_coordy_ma1(&self) -> SwStCoordyMa1R {
        SwStCoordyMa1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:22 - the start y-coordinate of mask area 1 of Vertical start pixel\n\nrange:sw_st_coordy_ma1~dst width"]
    #[inline(always)]
    pub fn sw_end_coordy_ma1(&self) -> SwEndCoordyMa1R {
        SwEndCoordyMa1R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - the start y-coordinate of mask area 1 of Vertical start pixel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_st_coordy_ma1(&mut self) -> SwStCoordyMa1W<Swreg27Spec> {
        SwStCoordyMa1W::new(self, 0)
    }
    #[doc = "Bits 12:22 - the start y-coordinate of mask area 1 of Vertical start pixel\n\nrange:sw_st_coordy_ma1~dst width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_end_coordy_ma1(&mut self) -> SwEndCoordyMa1W<Swreg27Spec> {
        SwEndCoordyMa1W::new(self, 12)
    }
}
#[doc = "y-coordinate of mask area 1 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg27Spec;
impl crate::RegisterSpec for Swreg27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg27::R`](R) reader structure"]
impl crate::Readable for Swreg27Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg27::W`](W) writer structure"]
impl crate::Writable for Swreg27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG27 to value 0"]
impl crate::Resettable for Swreg27Spec {
    const RESET_VALUE: u32 = 0;
}
