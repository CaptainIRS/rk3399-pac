#[doc = "Register `SWREG8` reader"]
pub type R = crate::R<Swreg8Spec>;
#[doc = "Register `SWREG8` writer"]
pub type W = crate::W<Swreg8Spec>;
#[doc = "Field `SW_PIXNUM_LEFT_BYD` reader - the Amount of vertical pixels beyond the left border\n\nRange : 0~dst_width"]
pub type SwPixnumLeftBydR = crate::FieldReader<u16>;
#[doc = "Field `SW_PIXNUM_LEFT_BYD` writer - the Amount of vertical pixels beyond the left border\n\nRange : 0~dst_width"]
pub type SwPixnumLeftBydW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_PIXNUM_RIGHT_BYD` reader - the Amount of vertical pixels beyond the right border\n\nRange : 0~dst_width"]
pub type SwPixnumRightBydR = crate::FieldReader<u16>;
#[doc = "Field `SW_PIXNUM_RIGHT_BYD` writer - the Amount of vertical pixels beyond the right border\n\nRange : 0~dst_width"]
pub type SwPixnumRightBydW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - the Amount of vertical pixels beyond the left border\n\nRange : 0~dst_width"]
    #[inline(always)]
    pub fn sw_pixnum_left_byd(&self) -> SwPixnumLeftBydR {
        SwPixnumLeftBydR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - the Amount of vertical pixels beyond the right border\n\nRange : 0~dst_width"]
    #[inline(always)]
    pub fn sw_pixnum_right_byd(&self) -> SwPixnumRightBydR {
        SwPixnumRightBydR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - the Amount of vertical pixels beyond the left border\n\nRange : 0~dst_width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pixnum_left_byd(&mut self) -> SwPixnumLeftBydW<Swreg8Spec> {
        SwPixnumLeftBydW::new(self, 0)
    }
    #[doc = "Bits 16:26 - the Amount of vertical pixels beyond the right border\n\nRange : 0~dst_width"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pixnum_right_byd(&mut self) -> SwPixnumRightBydW<Swreg8Spec> {
        SwPixnumRightBydW::new(self, 16)
    }
}
#[doc = "Amount of pixels beyond border\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg8Spec;
impl crate::RegisterSpec for Swreg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg8::R`](R) reader structure"]
impl crate::Readable for Swreg8Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg8::W`](W) writer structure"]
impl crate::Writable for Swreg8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG8 to value 0"]
impl crate::Resettable for Swreg8Spec {
    const RESET_VALUE: u32 = 0;
}
