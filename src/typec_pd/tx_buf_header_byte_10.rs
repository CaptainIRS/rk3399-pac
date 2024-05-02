#[doc = "Register `TX_BUF_HEADER_BYTE_10` reader"]
pub type R = crate::R<TxBufHeaderByte10Spec>;
#[doc = "Register `TX_BUF_HEADER_BYTE_10` writer"]
pub type W = crate::W<TxBufHeaderByte10Spec>;
#[doc = "Field `TX_BUF_HEADER_BYTE_0` reader - Byte 0 (bits 7..0) of USB PD message \n\nheader"]
pub type TxBufHeaderByte0R = crate::FieldReader;
#[doc = "Field `TX_BUF_HEADER_BYTE_0` writer - Byte 0 (bits 7..0) of USB PD message \n\nheader"]
pub type TxBufHeaderByte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_BUF_HEADER_BYTE_1` reader - Byte 1 (bits 15..8) of USB PD message \n\nheader"]
pub type TxBufHeaderByte1R = crate::FieldReader;
#[doc = "Field `TX_BUF_HEADER_BYTE_1` writer - Byte 1 (bits 15..8) of USB PD message \n\nheader"]
pub type TxBufHeaderByte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value"]
pub type NotUsedR = crate::FieldReader<u16>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value"]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Byte 0 (bits 7..0) of USB PD message \n\nheader"]
    #[inline(always)]
    pub fn tx_buf_header_byte_0(&self) -> TxBufHeaderByte0R {
        TxBufHeaderByte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 1 (bits 15..8) of USB PD message \n\nheader"]
    #[inline(always)]
    pub fn tx_buf_header_byte_1(&self) -> TxBufHeaderByte1R {
        TxBufHeaderByte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value"]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte 0 (bits 7..0) of USB PD message \n\nheader"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_header_byte_0(&mut self) -> TxBufHeaderByte0W<TxBufHeaderByte10Spec> {
        TxBufHeaderByte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Byte 1 (bits 15..8) of USB PD message \n\nheader"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_header_byte_1(&mut self) -> TxBufHeaderByte1W<TxBufHeaderByte10Spec> {
        TxBufHeaderByte1W::new(self, 8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value"]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<TxBufHeaderByte10Spec> {
        NotUsedW::new(self, 16)
    }
}
#[doc = "Transmit Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_buf_header_byte_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_buf_header_byte_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBufHeaderByte10Spec;
impl crate::RegisterSpec for TxBufHeaderByte10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_buf_header_byte_10::R`](R) reader structure"]
impl crate::Readable for TxBufHeaderByte10Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_buf_header_byte_10::W`](W) writer structure"]
impl crate::Writable for TxBufHeaderByte10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_BUF_HEADER_BYTE_10 to value 0"]
impl crate::Resettable for TxBufHeaderByte10Spec {
    const RESET_VALUE: u32 = 0;
}
