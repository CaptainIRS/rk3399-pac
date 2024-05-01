#[doc = "Register `HRDMAL` reader"]
pub type R = crate::R<HrdmalSpec>;
#[doc = "Register `HRDMAL` writer"]
pub type W = crate::W<HrdmalSpec>;
#[doc = "Field `LENGTH` reader - Specifies the Block length of DMA.\n\nThe length unit is BYTE."]
pub type LengthR = crate::FieldReader<u32>;
#[doc = "Field `LENGTH` writer - Specifies the Block length of DMA.\n\nThe length unit is BYTE."]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the Block length of DMA.\n\nThe length unit is BYTE."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the Block length of DMA.\n\nThe length unit is BYTE."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<HrdmalSpec> {
        LengthW::new(self, 0)
    }
}
#[doc = "Hash Receiving DMA Length Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hrdmal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hrdmal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrdmalSpec;
impl crate::RegisterSpec for HrdmalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrdmal::R`](R) reader structure"]
impl crate::Readable for HrdmalSpec {}
#[doc = "`write(|w| ..)` method takes [`hrdmal::W`](W) writer structure"]
impl crate::Writable for HrdmalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HRDMAL to value 0"]
impl crate::Resettable for HrdmalSpec {
    const RESET_VALUE: u32 = 0;
}
