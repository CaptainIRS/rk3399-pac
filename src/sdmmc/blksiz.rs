#[doc = "Register `BLKSIZ` reader"]
pub type R = crate::R<BlksizSpec>;
#[doc = "Register `BLKSIZ` writer"]
pub type W = crate::W<BlksizSpec>;
#[doc = "Field `BLOCK_SIZE` reader - Block size"]
pub type BlockSizeR = crate::FieldReader<u16>;
#[doc = "Field `BLOCK_SIZE` writer - Block size"]
pub type BlockSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Block size"]
    #[inline(always)]
    pub fn block_size(&self) -> BlockSizeR {
        BlockSizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Block size"]
    #[inline(always)]
    #[must_use]
    pub fn block_size(&mut self) -> BlockSizeW<BlksizSpec> {
        BlockSizeW::new(self, 0)
    }
}
#[doc = "Block-size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blksiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blksiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlksizSpec;
impl crate::RegisterSpec for BlksizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blksiz::R`](R) reader structure"]
impl crate::Readable for BlksizSpec {}
#[doc = "`write(|w| ..)` method takes [`blksiz::W`](W) writer structure"]
impl crate::Writable for BlksizSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLKSIZ to value 0x0200"]
impl crate::Resettable for BlksizSpec {
    const RESET_VALUE: u32 = 0x0200;
}
