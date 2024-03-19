#[doc = "Register `HDCP_KSV[%s]` reader"]
pub type R = crate::R<HdcpKsvSpec>;
#[doc = "Register `HDCP_KSV[%s]` writer"]
pub type W = crate::W<HdcpKsvSpec>;
#[doc = "Field `HDCP_KSV_BYTE` reader - Sink KSV FIFO byte, ordered in little endian (byte at\n\naddress 0x502a belongs to byte 0 of KSV0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space, 635 KSV FIFO bytes are\n\nmapped, which allow for 127 KSV values, each with\n\n5 bytes (40 bits)."]
pub type HdcpKsvByteR = crate::FieldReader;
#[doc = "Field `HDCP_KSV_BYTE` writer - Sink KSV FIFO byte, ordered in little endian (byte at\n\naddress 0x502a belongs to byte 0 of KSV0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space, 635 KSV FIFO bytes are\n\nmapped, which allow for 127 KSV values, each with\n\n5 bytes (40 bits)."]
pub type HdcpKsvByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sink KSV FIFO byte, ordered in little endian (byte at\n\naddress 0x502a belongs to byte 0 of KSV0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space, 635 KSV FIFO bytes are\n\nmapped, which allow for 127 KSV values, each with\n\n5 bytes (40 bits)."]
    #[inline(always)]
    pub fn hdcp_ksv_byte(&self) -> HdcpKsvByteR {
        HdcpKsvByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sink KSV FIFO byte, ordered in little endian (byte at\n\naddress 0x502a belongs to byte 0 of KSV0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space, 635 KSV FIFO bytes are\n\nmapped, which allow for 127 KSV values, each with\n\n5 bytes (40 bits)."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_ksv_byte(&mut self) -> HdcpKsvByteW<HdcpKsvSpec> {
        HdcpKsvByteW::new(self, 0)
    }
}
#[doc = "HDCP KSV Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_ksv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_ksv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpKsvSpec;
impl crate::RegisterSpec for HdcpKsvSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_ksv::R`](R) reader structure"]
impl crate::Readable for HdcpKsvSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_ksv::W`](W) writer structure"]
impl crate::Writable for HdcpKsvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_KSV[%s]
to value 0xff"]
impl crate::Resettable for HdcpKsvSpec {
    const RESET_VALUE: u8 = 0xff;
}
