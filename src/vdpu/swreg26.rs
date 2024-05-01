#[doc = "Register `SWREG26` reader"]
pub type R = crate::R<Swreg26Spec>;
#[doc = "Register `SWREG26` writer"]
pub type W = crate::W<Swreg26Spec>;
#[doc = "Field `SW_ST_COORDX_MA1` reader - the start x-coordinate of mask area 1 of Horizontal start pixel"]
pub type SwStCoordxMa1R = crate::FieldReader<u16>;
#[doc = "Field `SW_ST_COORDX_MA1` writer - the start x-coordinate of mask area 1 of Horizontal start pixel"]
pub type SwStCoordxMa1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_END_COORDX_MA1` reader - the end x-coordinate of mask area 1 of Horizontal start pixel\n\nrange:sw_st_coordx_ma1-dst width"]
pub type SwEndCoordxMa1R = crate::FieldReader<u16>;
#[doc = "Field `SW_END_COORDX_MA1` writer - the end x-coordinate of mask area 1 of Horizontal start pixel\n\nrange:sw_st_coordx_ma1-dst width"]
pub type SwEndCoordxMa1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - the start x-coordinate of mask area 1 of Horizontal start pixel"]
    #[inline(always)]
    pub fn sw_st_coordx_ma1(&self) -> SwStCoordxMa1R {
        SwStCoordxMa1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 12:22 - the end x-coordinate of mask area 1 of Horizontal start pixel\n\nrange:sw_st_coordx_ma1-dst width"]
    #[inline(always)]
    pub fn sw_end_coordx_ma1(&self) -> SwEndCoordxMa1R {
        SwEndCoordxMa1R::new(((self.bits >> 12) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - the start x-coordinate of mask area 1 of Horizontal start pixel"]
    #[inline(always)]
    #[must_use]
    pub fn sw_st_coordx_ma1(&mut self) -> SwStCoordxMa1W<Swreg26Spec> {
        SwStCoordxMa1W::new(self, 0)
    }
    #[doc = "Bits 12:22 - the end x-coordinate of mask area 1 of Horizontal start pixel\n\nrange:sw_st_coordx_ma1-dst width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_end_coordx_ma1(&mut self) -> SwEndCoordxMa1W<Swreg26Spec> {
        SwEndCoordxMa1W::new(self, 12)
    }
}
#[doc = "x-coordinate of mask area 1 for Horizontal start pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg26::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg26::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg26Spec;
impl crate::RegisterSpec for Swreg26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg26::R`](R) reader structure"]
impl crate::Readable for Swreg26Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg26::W`](W) writer structure"]
impl crate::Writable for Swreg26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG26 to value 0"]
impl crate::Resettable for Swreg26Spec {
    const RESET_VALUE: u32 = 0;
}
