#[doc = "Register `TRANSMIT_BYTE_COUNT` reader"]
pub type R = crate::R<TransmitByteCountSpec>;
#[doc = "Register `TRANSMIT_BYTE_COUNT` writer"]
pub type W = crate::W<TransmitByteCountSpec>;
#[doc = "Field `TRANSMIT_BYTE_COUNT` reader - The number of bytes the TCPM \n\nwill write This is the number of \n\nbytes in the \n\nTX_BUFFER_DATA_OBJECTS \n\nplus \n\ntwo (for the TX_BUF_HEADER)."]
pub type TransmitByteCountR = crate::FieldReader;
#[doc = "Field `TRANSMIT_BYTE_COUNT` writer - The number of bytes the TCPM \n\nwill write This is the number of \n\nbytes in the \n\nTX_BUFFER_DATA_OBJECTS \n\nplus \n\ntwo (for the TX_BUF_HEADER)."]
pub type TransmitByteCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - The number of bytes the TCPM \n\nwill write This is the number of \n\nbytes in the \n\nTX_BUFFER_DATA_OBJECTS \n\nplus \n\ntwo (for the TX_BUF_HEADER)."]
    #[inline(always)]
    pub fn transmit_byte_count(&self) -> TransmitByteCountR {
        TransmitByteCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - The number of bytes the TCPM \n\nwill write This is the number of \n\nbytes in the \n\nTX_BUFFER_DATA_OBJECTS \n\nplus \n\ntwo (for the TX_BUF_HEADER)."]
    #[inline(always)]
    #[must_use]
    pub fn transmit_byte_count(&mut self) -> TransmitByteCountW<TransmitByteCountSpec> {
        TransmitByteCountW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<TransmitByteCountSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Transmit Byte Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`transmit_byte_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`transmit_byte_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TransmitByteCountSpec;
impl crate::RegisterSpec for TransmitByteCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`transmit_byte_count::R`](R) reader structure"]
impl crate::Readable for TransmitByteCountSpec {}
#[doc = "`write(|w| ..)` method takes [`transmit_byte_count::W`](W) writer structure"]
impl crate::Writable for TransmitByteCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRANSMIT_BYTE_COUNT to value 0"]
impl crate::Resettable for TransmitByteCountSpec {
    const RESET_VALUE: u32 = 0;
}
