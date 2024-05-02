#[doc = "Register `RX_BUF_HEADER_BYTE_10` reader"]
pub type R = crate::R<RxBufHeaderByte10Spec>;
#[doc = "Field `RX_BUF_HEADER_BYTE_0` reader - Byte 0 (bits 7..0) of USB PD message \n\nheader"]
pub type RxBufHeaderByte0R = crate::FieldReader;
#[doc = "Field `RX_BUF_HEADER_BYTE_1` reader - Byte 1 (bits 15..8) of USB PD message \n\nheader"]
pub type RxBufHeaderByte1R = crate::FieldReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Byte 0 (bits 7..0) of USB PD message \n\nheader"]
    #[inline(always)]
    pub fn rx_buf_header_byte_0(&self) -> RxBufHeaderByte0R {
        RxBufHeaderByte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 1 (bits 15..8) of USB PD message \n\nheader"]
    #[inline(always)]
    pub fn rx_buf_header_byte_1(&self) -> RxBufHeaderByte1R {
        RxBufHeaderByte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "RX Buffer Message Header Byte Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_buf_header_byte_10::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxBufHeaderByte10Spec;
impl crate::RegisterSpec for RxBufHeaderByte10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_buf_header_byte_10::R`](R) reader structure"]
impl crate::Readable for RxBufHeaderByte10Spec {}
#[doc = "`reset()` method sets RX_BUF_HEADER_BYTE_10 to value 0"]
impl crate::Resettable for RxBufHeaderByte10Spec {
    const RESET_VALUE: u32 = 0;
}
