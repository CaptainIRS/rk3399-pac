#[doc = "Register `CT_OFFSET_R` reader"]
pub type R = crate::R<CtOffsetRSpec>;
#[doc = "Register `CT_OFFSET_R` writer"]
pub type W = crate::W<CtOffsetRSpec>;
#[doc = "Field `ct_offset_r` reader - Offset red for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000.\n\n"]
pub type CtOffsetRR = crate::FieldReader<u16>;
#[doc = "Field `ct_offset_r` writer - Offset red for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000.\n\n"]
pub type CtOffsetRW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Offset red for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000.\n\n"]
    #[inline(always)]
    pub fn ct_offset_r(&self) -> CtOffsetRR {
        CtOffsetRR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset red for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn ct_offset_r(&mut self) -> CtOffsetRW<CtOffsetRSpec> {
        CtOffsetRW::new(self, 0)
    }
}
#[doc = "cross-talk offset red\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_offset_r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_offset_r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtOffsetRSpec;
impl crate::RegisterSpec for CtOffsetRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct_offset_r::R`](R) reader structure"]
impl crate::Readable for CtOffsetRSpec {}
#[doc = "`write(|w| ..)` method takes [`ct_offset_r::W`](W) writer structure"]
impl crate::Writable for CtOffsetRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT_OFFSET_R to value 0"]
impl crate::Resettable for CtOffsetRSpec {
    const RESET_VALUE: u32 = 0;
}
