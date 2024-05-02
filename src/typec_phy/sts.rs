#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `FIELD1` reader - Reserved"]
pub type Field1R = crate::FieldReader<u16>;
#[doc = "Field `FIELD0` reader - PHY APB access timeout: When set, an APB read/write request to PHY \n\nregisters failed (i.e. timed out). When set, this bit is cleared upon read.\n\nThe field is **cleared** (set to zero) following a read operation."]
pub type Field0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:14 - Reserved"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(self.bits & 0x7fff)
    }
    #[doc = "Bit 15 - PHY APB access timeout: When set, an APB read/write request to PHY \n\nregisters failed (i.e. timed out). When set, this bit is cleared upon read."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "PHY status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u16 = 0;
}
