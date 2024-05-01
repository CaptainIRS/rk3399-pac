#[doc = "Register `SWREG1` reader"]
pub type R = crate::R<Swreg1Spec>;
#[doc = "Register `SWREG1` writer"]
pub type W = crate::W<Swreg1Spec>;
#[doc = "Field `SW_COE_1ST_0` reader - used for all color components calculate,used together with y pix"]
pub type SwCoe1st0R = crate::FieldReader<u16>;
#[doc = "Field `SW_COE_1ST_0` writer - used for all color components calculate,used together with y pix"]
pub type SwCoe1st0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_COE_1ST_1` reader - used in all color components calculate,used together with y pix\n\nused for all color components calculate,used together with y pix"]
pub type SwCoe1st1R = crate::FieldReader<u16>;
#[doc = "Field `SW_COE_1ST_1` writer - used in all color components calculate,used together with y pix\n\nused for all color components calculate,used together with y pix"]
pub type SwCoe1st1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SW_COE_2ST` reader - used in red color components calculate,used together with cr pix\n\nused for red color components calculate,used together with cr pix"]
pub type SwCoe2stR = crate::FieldReader<u16>;
#[doc = "Field `SW_COE_2ST` writer - used in red color components calculate,used together with cr pix\n\nused for red color components calculate,used together with cr pix"]
pub type SwCoe2stW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - used for all color components calculate,used together with y pix"]
    #[inline(always)]
    pub fn sw_coe_1st_0(&self) -> SwCoe1st0R {
        SwCoe1st0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - used in all color components calculate,used together with y pix\n\nused for all color components calculate,used together with y pix"]
    #[inline(always)]
    pub fn sw_coe_1st_1(&self) -> SwCoe1st1R {
        SwCoe1st1R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - used in red color components calculate,used together with cr pix\n\nused for red color components calculate,used together with cr pix"]
    #[inline(always)]
    pub fn sw_coe_2st(&self) -> SwCoe2stR {
        SwCoe2stR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - used for all color components calculate,used together with y pix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_1st_0(&mut self) -> SwCoe1st0W<Swreg1Spec> {
        SwCoe1st0W::new(self, 0)
    }
    #[doc = "Bits 10:19 - used in all color components calculate,used together with y pix\n\nused for all color components calculate,used together with y pix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_1st_1(&mut self) -> SwCoe1st1W<Swreg1Spec> {
        SwCoe1st1W::new(self, 10)
    }
    #[doc = "Bits 20:29 - used in red color components calculate,used together with cr pix\n\nused for red color components calculate,used together with cr pix"]
    #[inline(always)]
    #[must_use]
    pub fn sw_coe_2st(&mut self) -> SwCoe2stW<Swreg1Spec> {
        SwCoe2stW::new(self, 20)
    }
}
#[doc = "color coeff register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg1Spec;
impl crate::RegisterSpec for Swreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg1::R`](R) reader structure"]
impl crate::Readable for Swreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg1::W`](W) writer structure"]
impl crate::Writable for Swreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG1 to value 0"]
impl crate::Resettable for Swreg1Spec {
    const RESET_VALUE: u32 = 0;
}
