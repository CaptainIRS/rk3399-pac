#[doc = "Register `HDCP_VH[%s]` reader"]
pub type R = crate::R<HdcpVhSpec>;
#[doc = "Register `HDCP_VH[%s]` writer"]
pub type W = crate::W<HdcpVhSpec>;
#[doc = "Field `HDCP_VH_BYTE` reader - Sink VH' byte, ordered in little endian (byte at\n\naddress 0x525a belongs to byte 0 of VH0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space 20 VH bytes are mapped,\n\nwhich allow for 5 VH values, each with 4 bytes\n\n(32bits)."]
pub type HdcpVhByteR = crate::FieldReader;
#[doc = "Field `HDCP_VH_BYTE` writer - Sink VH' byte, ordered in little endian (byte at\n\naddress 0x525a belongs to byte 0 of VH0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space 20 VH bytes are mapped,\n\nwhich allow for 5 VH values, each with 4 bytes\n\n(32bits)."]
pub type HdcpVhByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sink VH' byte, ordered in little endian (byte at\n\naddress 0x525a belongs to byte 0 of VH0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space 20 VH bytes are mapped,\n\nwhich allow for 5 VH values, each with 4 bytes\n\n(32bits)."]
    #[inline(always)]
    pub fn hdcp_vh_byte(&self) -> HdcpVhByteR {
        HdcpVhByteR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sink VH' byte, ordered in little endian (byte at\n\naddress 0x525a belongs to byte 0 of VH0). If Attr\n\nhas not been granted (see register a_ksvmemctrl),\n\nthe value read is 8'hff.\n\nIn this address space 20 VH bytes are mapped,\n\nwhich allow for 5 VH values, each with 4 bytes\n\n(32bits)."]
    #[inline(always)]
    #[must_use]
    pub fn hdcp_vh_byte(&mut self) -> HdcpVhByteW<HdcpVhSpec> {
        HdcpVhByteW::new(self, 0)
    }
}
#[doc = "HDCP SHA-1 VH Registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdcp_vh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdcp_vh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdcpVhSpec;
impl crate::RegisterSpec for HdcpVhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hdcp_vh::R`](R) reader structure"]
impl crate::Readable for HdcpVhSpec {}
#[doc = "`write(|w| ..)` method takes [`hdcp_vh::W`](W) writer structure"]
impl crate::Writable for HdcpVhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HDCP_VH[%s]
to value 0xff"]
impl crate::Resettable for HdcpVhSpec {
    const RESET_VALUE: u8 = 0xff;
}
