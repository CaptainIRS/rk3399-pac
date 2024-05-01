#[doc = "Register `SWREG12_VP9_REFERGOLDEN_BASE` reader"]
pub type R = crate::R<Swreg12Vp9RefergoldenBaseSpec>;
#[doc = "Register `SWREG12_VP9_REFERGOLDEN_BASE` writer"]
pub type W = crate::W<Swreg12Vp9RefergoldenBaseSpec>;
#[doc = "Field `SW_VP9GOLDEN_BASE` reader - base address for golden frame\n\nbase address for golden (the address should be 128bit align)"]
pub type SwVp9goldenBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_VP9GOLDEN_BASE` writer - base address for golden frame\n\nbase address for golden (the address should be 128bit align)"]
pub type SwVp9goldenBaseW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 4:31 - base address for golden frame\n\nbase address for golden (the address should be 128bit align)"]
    #[inline(always)]
    pub fn sw_vp9golden_base(&self) -> SwVp9goldenBaseR {
        SwVp9goldenBaseR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 4:31 - base address for golden frame\n\nbase address for golden (the address should be 128bit align)"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9golden_base(&mut self) -> SwVp9goldenBaseW<Swreg12Vp9RefergoldenBaseSpec> {
        SwVp9goldenBaseW::new(self, 4)
    }
}
#[doc = "base address for golden frame\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg12_vp9_refergolden_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg12_vp9_refergolden_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg12Vp9RefergoldenBaseSpec;
impl crate::RegisterSpec for Swreg12Vp9RefergoldenBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg12_vp9_refergolden_base::R`](R) reader structure"]
impl crate::Readable for Swreg12Vp9RefergoldenBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg12_vp9_refergolden_base::W`](W) writer structure"]
impl crate::Writable for Swreg12Vp9RefergoldenBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG12_VP9_REFERGOLDEN_BASE to value 0"]
impl crate::Resettable for Swreg12Vp9RefergoldenBaseSpec {
    const RESET_VALUE: u32 = 0;
}
