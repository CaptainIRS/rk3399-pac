#[doc = "Register `HDCP_REVOC_LIST[%s]` reader"]
pub type R = crate::R<HdcpRevocListSpec>;
#[doc = "Register `HDCP_REVOC_LIST[%s]` writer"]
pub type W = crate::W<HdcpRevocListSpec>;
#[doc = "Field `HDCP_REVOC_LIST_KSV_BYTE` reader - Revocation KSV byte, ordered in little endian (byte\n\nat address 0x52bb belongs to byte 0 of the first\n\nrevoked KSV). If Attr has not been granted (see\n\nregister a_ksvmemctrl), the value read is 8'hff.\n\nIn this address space 5060 revoked KSV bytes are\n\nmapped, which allow for 1012 KSV values, each\n\nwith 5 bytes (40 bits)."]
pub type HdcpRevocListKsvByteR = crate::FieldReader;
#[doc = "Field `HDCP_REVOC_LIST_KSV_BYTE` writer - Revocation KSV byte, ordered in little endian (byte\n\nat address 0x52bb belongs to byte 0 of the first\n\nrevoked KSV). If Attr has not been granted (see\n\nregister a_ksvmemctrl), the value read is 8'hff.\n\nIn this address space 5060 revoked KSV bytes are\n\nmapped, which allow for 1012 KSV values, each\n\nwith 5 bytes (40 bits)."]
pub type HdcpRevocListKsvByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Revocation KSV byte, ordered in little endian (byte\n\nat address 0x52bb belongs to byte 0 of the first\n\nrevoked KSV). If Attr has not been granted (see\n\nregister a_ksvmemctrl), the value read is 8'hff.\n\nIn this address space 5060 revoked KSV bytes are\n\nmapped, which allow for 1012 KSV values, each\n\nwith 5 bytes (40 bits)."]
    #[inline(always)]
    pub fn hdcp_revoc_list_ksv_byte(&self) -> HdcpRevocListKsvByteR {
        HdcpRevocListKsvByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Revocation KSV byte, ordered in little endian (byte\n\nat address 0x52bb belongs to byte 0 of the first\n\nrevoked KSV). If Attr has not been granted (see\n\nregister a_ksvmemctrl), the value read is 8'hff.\n\nIn this address space 5060 revoked KSV bytes are\n\nmapped, which allow for 1012 KSV values, each\n\nwith 5 bytes (40 bits)."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_revoc_list_ksv_byte(&mut self) -> HdcpRevocListKsvByteW<HdcpRevocListSpec> {
        HdcpRevocListKsvByteW::new(self, 0)
    }
}
#[doc = "HDCP Revocation KSV Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_revoc_list::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_revoc_list::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpRevocListSpec;
impl crate::RegisterSpec for HdcpRevocListSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_revoc_list::R`](R) reader structure"]
impl crate::Readable for HdcpRevocListSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_revoc_list::W`](W) writer structure"]
impl crate::Writable for HdcpRevocListSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_REVOC_LIST[%s]
to value 0xff"]
impl crate::Resettable for HdcpRevocListSpec {
    const RESET_VALUE: u8 = 0xff;
}
