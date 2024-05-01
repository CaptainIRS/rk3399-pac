#[doc = "Register `SWREG7` reader"]
pub type R = crate::R<Swreg7Spec>;
#[doc = "Register `SWREG7` writer"]
pub type W = crate::W<Swreg7Spec>;
#[doc = "Field `SW_PIXNUM_UP_BYD` reader - the Amount of vertical pixels beyond the up border\n\nRange : 0-dst_height"]
pub type SwPixnumUpBydR = crate::FieldReader<u16>;
#[doc = "Field `SW_PIXNUM_UP_BYD` writer - the Amount of vertical pixels beyond the up border\n\nRange : 0-dst_height"]
pub type SwPixnumUpBydW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SW_PIXNUM_DOWN_BYD` reader - the Amount of vertical pixels beyond the down border\n\nRange : 0-dst_height"]
pub type SwPixnumDownBydR = crate::FieldReader<u16>;
#[doc = "Field `SW_PIXNUM_DOWN_BYD` writer - the Amount of vertical pixels beyond the down border\n\nRange : 0-dst_height"]
pub type SwPixnumDownBydW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - the Amount of vertical pixels beyond the up border\n\nRange : 0-dst_height"]
    #[inline(always)]
    pub fn sw_pixnum_up_byd(&self) -> SwPixnumUpBydR {
        SwPixnumUpBydR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - the Amount of vertical pixels beyond the down border\n\nRange : 0-dst_height"]
    #[inline(always)]
    pub fn sw_pixnum_down_byd(&self) -> SwPixnumDownBydR {
        SwPixnumDownBydR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - the Amount of vertical pixels beyond the up border\n\nRange : 0-dst_height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pixnum_up_byd(&mut self) -> SwPixnumUpBydW<Swreg7Spec> {
        SwPixnumUpBydW::new(self, 0)
    }
    #[doc = "Bits 16:26 - the Amount of vertical pixels beyond the down border\n\nRange : 0-dst_height"]
    #[inline(always)]
    #[must_use]
    pub fn sw_pixnum_down_byd(&mut self) -> SwPixnumDownBydW<Swreg7Spec> {
        SwPixnumDownBydW::new(self, 16)
    }
}
#[doc = "Amount of pixels beyond border\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg7Spec;
impl crate::RegisterSpec for Swreg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg7::R`](R) reader structure"]
impl crate::Readable for Swreg7Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg7::W`](W) writer structure"]
impl crate::Writable for Swreg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG7 to value 0"]
impl crate::Resettable for Swreg7Spec {
    const RESET_VALUE: u32 = 0;
}
