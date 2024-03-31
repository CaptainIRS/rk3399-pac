#[doc = "Register `DBGSTATUS` reader"]
pub type R = crate::R<DbgstatusSpec>;
#[doc = "Field `DBGSTATUS_BITS_1` reader - The debug encoding is as follows:\n\nb00 = execute the instruction that the DBGINST \\[1:0\\]
Registers\n\ncontain\n\nb01 = reserved\n\nb10 = reserved\n\nb11 = reserved."]
pub type DbgstatusBits1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - The debug encoding is as follows:\n\nb00 = execute the instruction that the DBGINST \\[1:0\\]
Registers\n\ncontain\n\nb01 = reserved\n\nb10 = reserved\n\nb11 = reserved."]
    #[inline(always)]
    pub fn dbgstatus_bits_1(&self) -> DbgstatusBits1R {
        DbgstatusBits1R::new((self.bits & 3) as u8)
    }
}
#[doc = "Debug Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgstatus::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgstatusSpec;
impl crate::RegisterSpec for DbgstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgstatus::R`](R) reader structure"]
impl crate::Readable for DbgstatusSpec {}
#[doc = "`reset()` method sets DBGSTATUS to value 0"]
impl crate::Resettable for DbgstatusSpec {
    const RESET_VALUE: u32 = 0;
}
