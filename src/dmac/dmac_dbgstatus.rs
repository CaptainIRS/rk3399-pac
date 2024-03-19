#[doc = "Register `DMAC_DBGSTATUS` reader"]
pub type R = crate::R<DmacDbgstatusSpec>;
#[doc = "Field `DMAC_DBGSTATUS_BITS_1` reader - The debug encoding is as follows:\n\nb00 = execute the instruction that the DBGINST \\[1:0\\]
Registers\n\ncontain\n\nb01 = reserved\n\nb10 = reserved\n\nb11 = reserved."]
pub type DmacDbgstatusBits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - The debug encoding is as follows:\n\nb00 = execute the instruction that the DBGINST \\[1:0\\]
Registers\n\ncontain\n\nb01 = reserved\n\nb10 = reserved\n\nb11 = reserved."]
    #[inline(always)]
    pub fn dmac_dbgstatus_bits_1(&self) -> DmacDbgstatusBits1R {
        DmacDbgstatusBits1R::new((self.bits & 3) as u8)
    }
}
#[doc = "Debug Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dbgstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacDbgstatusSpec;
impl crate::RegisterSpec for DmacDbgstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_dbgstatus::R`](R) reader structure"]
impl crate::Readable for DmacDbgstatusSpec {}
#[doc = "`reset()` method sets DMAC_DBGSTATUS to value 0"]
impl crate::Resettable for DmacDbgstatusSpec {
    const RESET_VALUE: u32 = 0;
}
