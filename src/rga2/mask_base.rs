#[doc = "Register `MASK_BASE` reader"]
pub type R = crate::R<MaskBaseSpec>;
#[doc = "Register `MASK_BASE` writer"]
pub type W = crate::W<MaskBaseSpec>;
#[doc = "Field `SW_MASK_BASE` reader - mask base address in ROP4 mode\n\nLUT/ pattern load base address"]
pub type SwMaskBaseR = crate::FieldReader<u32>;
#[doc = "Field `SW_MASK_BASE` writer - mask base address in ROP4 mode\n\nLUT/ pattern load base address"]
pub type SwMaskBaseW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - mask base address in ROP4 mode\n\nLUT/ pattern load base address"]
    #[inline(always)]
    pub fn sw_mask_base(&self) -> SwMaskBaseR {
        SwMaskBaseR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - mask base address in ROP4 mode\n\nLUT/ pattern load base address"]
    #[inline(always)]
    #[must_use]
    pub fn sw_mask_base(&mut self) -> SwMaskBaseW<MaskBaseSpec> {
        SwMaskBaseW::new(self, 0)
    }
}
#[doc = "RGA mask base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mask_base::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mask_base::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskBaseSpec;
impl crate::RegisterSpec for MaskBaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mask_base::R`](R) reader structure"]
impl crate::Readable for MaskBaseSpec {}
#[doc = "`write(|w| ..)` method takes [`mask_base::W`](W) writer structure"]
impl crate::Writable for MaskBaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASK_BASE to value 0"]
impl crate::Resettable for MaskBaseSpec {
    const RESET_VALUE: u32 = 0;
}
