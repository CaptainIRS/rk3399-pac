#[doc = "Register `SWREG16_VP9_SEGIDCUR_BASE` reader"]
pub type R = crate::R<Swreg16Vp9SegidcurBaseSpec>;
#[doc = "Register `SWREG16_VP9_SEGIDCUR_BASE` writer"]
pub type W = crate::W<Swreg16Vp9SegidcurBaseSpec>;
#[doc = "Field `SW_VP9SEGIDCUR_BASE` reader - base address for vp9 cur frame segment id\n\nbase address for vp9 cur frame segment id (the address should be\n\n128bit align)"]
pub type SwVp9segidcurBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9SEGIDCUR_BASE` writer - base address for vp9 cur frame segment id\n\nbase address for vp9 cur frame segment id (the address should be\n\n128bit align)"]
pub type SwVp9segidcurBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for vp9 cur frame segment id\n\nbase address for vp9 cur frame segment id (the address should be\n\n128bit align)"]
    #[inline(always)]
    pub fn sw_vp9segidcur_base(&self) -> SwVp9segidcurBaseR {
        SwVp9segidcurBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for vp9 cur frame segment id\n\nbase address for vp9 cur frame segment id (the address should be\n\n128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segidcur_base(&mut self) -> SwVp9segidcurBaseW<Swreg16Vp9SegidcurBaseSpec> {
        SwVp9segidcurBaseW::new(self, 4)
    }
}
#[doc = "base address for cur frame segment id\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg16_vp9_segidcur_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg16_vp9_segidcur_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg16Vp9SegidcurBaseSpec;
impl crate::RegisterSpec for Swreg16Vp9SegidcurBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg16_vp9_segidcur_base::R`](R) reader structure"]
impl crate::Readable for Swreg16Vp9SegidcurBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg16_vp9_segidcur_base::W`](W) writer structure"]
impl crate::Writable for Swreg16Vp9SegidcurBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG16_VP9_SEGIDCUR_BASE to value 0"]
impl crate::Resettable for Swreg16Vp9SegidcurBaseSpec {
    const RESET_VALUE: u32 = 0;
}
