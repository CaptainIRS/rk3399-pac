#[doc = "Register `BYTCNT` reader"]
pub type R = crate::R<BytcntSpec>;
#[doc = "Register `BYTCNT` writer"]
pub type W = crate::W<BytcntSpec>;
#[doc = "Field `BYTE_COUNT` reader - Number of bytes to be transferred; should be integer multiple of\n\nBlock Size for block transfers.\n\nFor undefined number of byte transfers, byte count should be set\n\nto 0. When byte count is set to 0, it is responsibility of host to\n\nexplicitly send stop/abort command to terminate data transfer."]
pub type ByteCountR = crate::FieldReader<u32>;
#[doc = "Field `BYTE_COUNT` writer - Number of bytes to be transferred; should be integer multiple of\n\nBlock Size for block transfers.\n\nFor undefined number of byte transfers, byte count should be set\n\nto 0. When byte count is set to 0, it is responsibility of host to\n\nexplicitly send stop/abort command to terminate data transfer."]
pub type ByteCountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes to be transferred; should be integer multiple of\n\nBlock Size for block transfers.\n\nFor undefined number of byte transfers, byte count should be set\n\nto 0. When byte count is set to 0, it is responsibility of host to\n\nexplicitly send stop/abort command to terminate data transfer."]
    #[inline(always)]
    pub fn byte_count(&self) -> ByteCountR {
        ByteCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes to be transferred; should be integer multiple of\n\nBlock Size for block transfers.\n\nFor undefined number of byte transfers, byte count should be set\n\nto 0. When byte count is set to 0, it is responsibility of host to\n\nexplicitly send stop/abort command to terminate data transfer."]
    #[inline(always)]
    #[must_use]
    pub fn byte_count(&mut self) -> ByteCountW<BytcntSpec> {
        ByteCountW::new(self, 0)
    }
}
#[doc = "Byte-count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BytcntSpec;
impl crate::RegisterSpec for BytcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bytcnt::R`](R) reader structure"]
impl crate::Readable for BytcntSpec {}
#[doc = "`write(|w| ..)` method takes [`bytcnt::W`](W) writer structure"]
impl crate::Writable for BytcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BYTCNT to value 0x0200"]
impl crate::Resettable for BytcntSpec {
    const RESET_VALUE: u32 = 0x0200;
}
