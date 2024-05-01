#[doc = "Register `SWREG13_VP9_REFERALFTER_BASE` reader"]
pub type R = crate::R<Swreg13Vp9ReferalfterBaseSpec>;
#[doc = "Register `SWREG13_VP9_REFERALFTER_BASE` writer"]
pub type W = crate::W<Swreg13Vp9ReferalfterBaseSpec>;
#[doc = "Field `SW_VP9ALFTER_BASE` reader - base address for alfter frame\n\nbase address for alfter (the address should be 128bit align)"]
pub type SwVp9alfterBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9ALFTER_BASE` writer - base address for alfter frame\n\nbase address for alfter (the address should be 128bit align)"]
pub type SwVp9alfterBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for alfter frame\n\nbase address for alfter (the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_vp9alfter_base(&self) -> SwVp9alfterBaseR {
        SwVp9alfterBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for alfter frame\n\nbase address for alfter (the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9alfter_base(&mut self) -> SwVp9alfterBaseW<Swreg13Vp9ReferalfterBaseSpec> {
        SwVp9alfterBaseW::new(self, 4)
    }
}
#[doc = "base address for referalfter frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg13_vp9_referalfter_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg13_vp9_referalfter_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg13Vp9ReferalfterBaseSpec;
impl crate::RegisterSpec for Swreg13Vp9ReferalfterBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg13_vp9_referalfter_base::R`](R) reader structure"]
impl crate::Readable for Swreg13Vp9ReferalfterBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg13_vp9_referalfter_base::W`](W) writer structure"]
impl crate::Writable for Swreg13Vp9ReferalfterBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG13_VP9_REFERALFTER_BASE to value 0"]
impl crate::Resettable for Swreg13Vp9ReferalfterBaseSpec {
    const RESET_VALUE: u32 = 0;
}
