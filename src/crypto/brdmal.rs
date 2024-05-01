#[doc = "Register `BRDMAL` reader"]
pub type R = crate::R<BrdmalSpec>;
#[doc = "Register `BRDMAL` writer"]
pub type W = crate::W<BrdmalSpec>;
#[doc = "Field `LENGTH` reader - Specifies the Block length of DMA.\n\nThe length unit is WORD."]
pub type LengthR = crate::FieldReader<u32>;
#[doc = "Field `LENGTH` writer - Specifies the Block length of DMA.\n\nThe length unit is WORD."]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the Block length of DMA.\n\nThe length unit is WORD."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the Block length of DMA.\n\nThe length unit is WORD."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<BrdmalSpec> {
        LengthW::new(self, 0)
    }
}
#[doc = "Block Receiving DMA Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brdmal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brdmal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrdmalSpec;
impl crate::RegisterSpec for BrdmalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brdmal::R`](R) reader structure"]
impl crate::Readable for BrdmalSpec {}
#[doc = "`write(|w| ..)` method takes [`brdmal::W`](W) writer structure"]
impl crate::Writable for BrdmalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BRDMAL to value 0"]
impl crate::Resettable for BrdmalSpec {
    const RESET_VALUE: u32 = 0;
}
