#[doc = "Register `INT_MASK` reader"]
pub type R = crate::R<IntMaskSpec>;
#[doc = "Register `INT_MASK` writer"]
pub type W = crate::W<IntMaskSpec>;
#[doc = "Field `INT_MASK` reader - Interrupt mask control, when bit set to 1'b1, the corresponding\n\ninterrupt will disable"]
pub type IntMaskR = crate::FieldReader<u16>;
#[doc = "Field `INT_MASK` writer - Interrupt mask control, when bit set to 1'b1, the corresponding\n\ninterrupt will disable"]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Interrupt mask control, when bit set to 1'b1, the corresponding\n\ninterrupt will disable"]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Interrupt mask control, when bit set to 1'b1, the corresponding\n\ninterrupt will disable"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask(&mut self) -> IntMaskW<IntMaskSpec> {
        IntMaskW::new(self, 0)
    }
}
#[doc = "Interrupt mask control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntMaskSpec;
impl crate::RegisterSpec for IntMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_mask::R`](R) reader structure"]
impl crate::Readable for IntMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_mask::W`](W) writer structure"]
impl crate::Writable for IntMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_MASK to value 0"]
impl crate::Resettable for IntMaskSpec {
    const RESET_VALUE: u32 = 0;
}
