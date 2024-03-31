#[doc = "Register `BLKCNT` reader"]
pub type R = crate::R<BlkcntSpec>;
#[doc = "Register `BLKCNT` writer"]
pub type W = crate::W<BlkcntSpec>;
#[doc = "Field `BLOCKCOUNTFORCURRENTTRANSFER` reader - This register is enabled when Block Count Enable in the Transfer\n\nMode register is set to 1 and is valid only for multiple block\n\ntransfers. The HC decrements the block count after each block\n\ntransfer and stops when the count reaches zero. It can be\n\naccessed only if no transaction is executing (i.e. after a\n\ntransaction has stopped). Read operations during transfer return\n\nan invalid value and write operations shall be ignored.\n\nWhen saving transfer context as a result of Suspend command,\n\nthe number of blocks yet to be transferred can be determined by\n\nreading this register. When restoring transfer context prior to\n\nissuing a Resume command, the HD shall restore the previously\n\nsave block count.\n\n16'h0000: Stop Count\n\n16'h0001: 1 block\n\n16'h0002: 2 blocks\n\n........\n\n16'hFFFF: 65535 blocks"]
pub type BlockcountforcurrenttransferR = crate::FieldReader<u16>;
#[doc = "Field `BLOCKCOUNTFORCURRENTTRANSFER` writer - This register is enabled when Block Count Enable in the Transfer\n\nMode register is set to 1 and is valid only for multiple block\n\ntransfers. The HC decrements the block count after each block\n\ntransfer and stops when the count reaches zero. It can be\n\naccessed only if no transaction is executing (i.e. after a\n\ntransaction has stopped). Read operations during transfer return\n\nan invalid value and write operations shall be ignored.\n\nWhen saving transfer context as a result of Suspend command,\n\nthe number of blocks yet to be transferred can be determined by\n\nreading this register. When restoring transfer context prior to\n\nissuing a Resume command, the HD shall restore the previously\n\nsave block count.\n\n16'h0000: Stop Count\n\n16'h0001: 1 block\n\n16'h0002: 2 blocks\n\n........\n\n16'hFFFF: 65535 blocks"]
pub type BlockcountforcurrenttransferW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register is enabled when Block Count Enable in the Transfer\n\nMode register is set to 1 and is valid only for multiple block\n\ntransfers. The HC decrements the block count after each block\n\ntransfer and stops when the count reaches zero. It can be\n\naccessed only if no transaction is executing (i.e. after a\n\ntransaction has stopped). Read operations during transfer return\n\nan invalid value and write operations shall be ignored.\n\nWhen saving transfer context as a result of Suspend command,\n\nthe number of blocks yet to be transferred can be determined by\n\nreading this register. When restoring transfer context prior to\n\nissuing a Resume command, the HD shall restore the previously\n\nsave block count.\n\n16'h0000: Stop Count\n\n16'h0001: 1 block\n\n16'h0002: 2 blocks\n\n........\n\n16'hFFFF: 65535 blocks"]
    #[inline(always)]
    pub fn blockcountforcurrenttransfer(&self) -> BlockcountforcurrenttransferR {
        BlockcountforcurrenttransferR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - This register is enabled when Block Count Enable in the Transfer\n\nMode register is set to 1 and is valid only for multiple block\n\ntransfers. The HC decrements the block count after each block\n\ntransfer and stops when the count reaches zero. It can be\n\naccessed only if no transaction is executing (i.e. after a\n\ntransaction has stopped). Read operations during transfer return\n\nan invalid value and write operations shall be ignored.\n\nWhen saving transfer context as a result of Suspend command,\n\nthe number of blocks yet to be transferred can be determined by\n\nreading this register. When restoring transfer context prior to\n\nissuing a Resume command, the HD shall restore the previously\n\nsave block count.\n\n16'h0000: Stop Count\n\n16'h0001: 1 block\n\n16'h0002: 2 blocks\n\n........\n\n16'hFFFF: 65535 blocks"]
    #[inline(always)]
    #[must_use]
    pub fn blockcountforcurrenttransfer(&mut self) -> BlockcountforcurrenttransferW<BlkcntSpec> {
        BlockcountforcurrenttransferW::new(self, 0)
    }
}
#[doc = "Block count register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blkcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blkcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BlkcntSpec;
impl crate::RegisterSpec for BlkcntSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`blkcnt::R`](R) reader structure"]
impl crate::Readable for BlkcntSpec {}
#[doc = "`write(|w| ..)` method takes [`blkcnt::W`](W) writer structure"]
impl crate::Writable for BlkcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BLKCNT to value 0"]
impl crate::Resettable for BlkcntSpec {
    const RESET_VALUE: u16 = 0;
}
