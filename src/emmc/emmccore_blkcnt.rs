#[doc = "Register `EMMCCORE_BLKCNT` reader"]
pub type R = crate::R<EmmccoreBlkcntSpec>;
#[doc = "Register `EMMCCORE_BLKCNT` writer"]
pub type W = crate::W<EmmccoreBlkcntSpec>;
#[doc = "Field `BLOCKCOUNTFORCURRENTTRANSFER` reader - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count. 16'h0000: Stop Count 16'h0001: 1 block 16'h0002: 2 blocks ........ 16'hFFFF: 65535 blocks"]
pub type BlockcountforcurrenttransferR = crate::FieldReader<u16>;
#[doc = "Field `BLOCKCOUNTFORCURRENTTRANSFER` writer - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count. 16'h0000: Stop Count 16'h0001: 1 block 16'h0002: 2 blocks ........ 16'hFFFF: 65535 blocks"]
pub type BlockcountforcurrenttransferW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count. 16'h0000: Stop Count 16'h0001: 1 block 16'h0002: 2 blocks ........ 16'hFFFF: 65535 blocks"]
    #[inline(always)]
    pub fn blockcountforcurrenttransfer(&self) -> BlockcountforcurrenttransferR {
        BlockcountforcurrenttransferR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The HC decrements the block count after each block transfer and stops when the count reaches zero. It can be accessed only if no transaction is executing (i.e. after a transaction has stopped). Read operations during transfer return an invalid value and write operations shall be ignored. When saving transfer context as a result of Suspend command, the number of blocks yet to be transferred can be determined by reading this register. When restoring transfer context prior to issuing a Resume command, the HD shall restore the previously save block count. 16'h0000: Stop Count 16'h0001: 1 block 16'h0002: 2 blocks ........ 16'hFFFF: 65535 blocks"]
    #[inline(always)]
    #[must_use]
    pub fn blockcountforcurrenttransfer(
        &mut self,
    ) -> BlockcountforcurrenttransferW<EmmccoreBlkcntSpec> {
        BlockcountforcurrenttransferW::new(self, 0)
    }
}
#[doc = "Block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_blkcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emmccore_blkcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreBlkcntSpec;
impl crate::RegisterSpec for EmmccoreBlkcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_blkcnt::R`](R) reader structure"]
impl crate::Readable for EmmccoreBlkcntSpec {}
#[doc = "`write(|w| ..)` method takes [`emmccore_blkcnt::W`](W) writer structure"]
impl crate::Writable for EmmccoreBlkcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EMMCCORE_BLKCNT to value 0"]
impl crate::Resettable for EmmccoreBlkcntSpec {
    const RESET_VALUE: u16 = 0;
}
