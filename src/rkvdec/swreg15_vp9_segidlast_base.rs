#[doc = "Register `SWREG15_VP9_SEGIDLAST_BASE` reader"]
pub type R = crate::R<Swreg15Vp9SegidlastBaseSpec>;
#[doc = "Register `SWREG15_VP9_SEGIDLAST_BASE` writer"]
pub type W = crate::W<Swreg15Vp9SegidlastBaseSpec>;
#[doc = "Field `SW_VP9SEGIDLAST_BASE` reader - base address for vp9 last frame segment id\n\nbase address for vp9 last frame segment id (the address should be\n\n128bit align)"]
pub type SwVp9segidlastBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9SEGIDLAST_BASE` writer - base address for vp9 last frame segment id\n\nbase address for vp9 last frame segment id (the address should be\n\n128bit align)"]
pub type SwVp9segidlastBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for vp9 last frame segment id\n\nbase address for vp9 last frame segment id (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_vp9segidlast_base(&self) -> SwVp9segidlastBaseR {
        SwVp9segidlastBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for vp9 last frame segment id\n\nbase address for vp9 last frame segment id (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segidlast_base(&mut self) -> SwVp9segidlastBaseW<Swreg15Vp9SegidlastBaseSpec> {
        SwVp9segidlastBaseW::new(self, 4)
    }
}
#[doc = "base address for last frame segment id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg15_vp9_segidlast_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg15_vp9_segidlast_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg15Vp9SegidlastBaseSpec;
impl crate::RegisterSpec for Swreg15Vp9SegidlastBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg15_vp9_segidlast_base::R`](R) reader structure"]
impl crate::Readable for Swreg15Vp9SegidlastBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg15_vp9_segidlast_base::W`](W) writer structure"]
impl crate::Writable for Swreg15Vp9SegidlastBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG15_VP9_SEGIDLAST_BASE to value 0"]
impl crate::Resettable for Swreg15Vp9SegidlastBaseSpec {
    const RESET_VALUE: u32 = 0;
}
