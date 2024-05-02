#[doc = "Register `CT_OFFSET_G` reader"]
pub type R = crate::R<CtOffsetGSpec>;
#[doc = "Register `CT_OFFSET_G` writer"]
pub type W = crate::W<CtOffsetGSpec>;
#[doc = "Field `ct_offset_g` reader - Offset green for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000."]
pub type CtOffsetGR = crate::FieldReader<u16>;
#[doc = "Field `ct_offset_g` writer - Offset green for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000."]
pub type CtOffsetGW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Offset green for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000."]
    #[inline(always)]
    pub fn ct_offset_g(&self) -> CtOffsetGR {
        CtOffsetGR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset green for cross talk matrix. Two's complement\n\ninteger number ranging from -2048 (0x800) to 2047\n\n(0x7FF). 0 is represented as 0x000."]
    #[inline(always)]
    #[must_use]
    pub fn ct_offset_g(&mut self) -> CtOffsetGW<CtOffsetGSpec> {
        CtOffsetGW::new(self, 0)
    }
}
#[doc = "cross-talk offset green\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_offset_g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_offset_g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtOffsetGSpec;
impl crate::RegisterSpec for CtOffsetGSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct_offset_g::R`](R) reader structure"]
impl crate::Readable for CtOffsetGSpec {}
#[doc = "`write(|w| ..)` method takes [`ct_offset_g::W`](W) writer structure"]
impl crate::Writable for CtOffsetGSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT_OFFSET_G to value 0"]
impl crate::Resettable for CtOffsetGSpec {
    const RESET_VALUE: u32 = 0;
}
