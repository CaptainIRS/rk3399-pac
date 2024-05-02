#[doc = "Register `CT_OFFSET_B` reader"]
pub type R = crate::R<CtOffsetBSpec>;
#[doc = "Register `CT_OFFSET_B` writer"]
pub type W = crate::W<CtOffsetBSpec>;
#[doc = "Field `ct_offset_b` reader - Offset blue for cross talk matrix. Two's\n\ncomplement integer number ranging from -2048\n\n(0x800) to 2047 (0x7FF). 0 is represented as 0x000.\n\n"]
pub type CtOffsetBR = crate::FieldReader<u16>;
#[doc = "Field `ct_offset_b` writer - Offset blue for cross talk matrix. Two's\n\ncomplement integer number ranging from -2048\n\n(0x800) to 2047 (0x7FF). 0 is represented as 0x000.\n\n"]
pub type CtOffsetBW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Offset blue for cross talk matrix. Two's\n\ncomplement integer number ranging from -2048\n\n(0x800) to 2047 (0x7FF). 0 is represented as 0x000.\n\n"]
    #[inline(always)]
    pub fn ct_offset_b(&self) -> CtOffsetBR {
        CtOffsetBR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset blue for cross talk matrix. Two's\n\ncomplement integer number ranging from -2048\n\n(0x800) to 2047 (0x7FF). 0 is represented as 0x000.\n\n"]
    #[inline(always)]
    #[must_use]
    pub fn ct_offset_b(&mut self) -> CtOffsetBW<CtOffsetBSpec> {
        CtOffsetBW::new(self, 0)
    }
}
#[doc = "cross-talk offset blue\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ct_offset_b::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ct_offset_b::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtOffsetBSpec;
impl crate::RegisterSpec for CtOffsetBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ct_offset_b::R`](R) reader structure"]
impl crate::Readable for CtOffsetBSpec {}
#[doc = "`write(|w| ..)` method takes [`ct_offset_b::W`](W) writer structure"]
impl crate::Writable for CtOffsetBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CT_OFFSET_B to value 0"]
impl crate::Resettable for CtOffsetBSpec {
    const RESET_VALUE: u32 = 0;
}
