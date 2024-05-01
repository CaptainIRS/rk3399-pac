#[doc = "Register `SWREG11_VP9_REFERLAST_BASE` reader"]
pub type R = crate::R<Swreg11Vp9ReferlastBaseSpec>;
#[doc = "Register `SWREG11_VP9_REFERLAST_BASE` writer"]
pub type W = crate::W<Swreg11Vp9ReferlastBaseSpec>;
#[doc = "Field `SW_VP9LAST_BASE` reader - base address for last frame\n\nbase address for last (the address should be 128bit align)"]
pub type SwVp9lastBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9LAST_BASE` writer - base address for last frame\n\nbase address for last (the address should be 128bit align)"]
pub type SwVp9lastBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for last frame\n\nbase address for last (the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_vp9last_base(&self) -> SwVp9lastBaseR {
        SwVp9lastBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for last frame\n\nbase address for last (the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9last_base(&mut self) -> SwVp9lastBaseW<Swreg11Vp9ReferlastBaseSpec> {
        SwVp9lastBaseW::new(self, 4)
    }
}
#[doc = "base address for last frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg11_vp9_referlast_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg11_vp9_referlast_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg11Vp9ReferlastBaseSpec;
impl crate::RegisterSpec for Swreg11Vp9ReferlastBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg11_vp9_referlast_base::R`](R) reader structure"]
impl crate::Readable for Swreg11Vp9ReferlastBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg11_vp9_referlast_base::W`](W) writer structure"]
impl crate::Writable for Swreg11Vp9ReferlastBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG11_VP9_REFERLAST_BASE to value 0"]
impl crate::Resettable for Swreg11Vp9ReferlastBaseSpec {
    const RESET_VALUE: u32 = 0;
}
