#[doc = "Register `RECEIVE_BYTE_COUNT` reader"]
pub type R = crate::R<ReceiveByteCountSpec>;
#[doc = "Field `RECEIVE_BYTE_COUNT` reader - Indicates number of bytes in this register \n\nthat are not stale. The TCPM should read \n\nthe first RECEIVE_BYTE_COUNT bytes in \n\nthis register. This is the number of bytes \n\nin the RX_BUFFER_DATA_OBJECTS plus \n\nthree (for the RX_BUF_FRAME_TYPE and \n\nRX_BUF_HEADER)."]
pub type ReceiveByteCountR = crate::FieldReader;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Indicates number of bytes in this register \n\nthat are not stale. The TCPM should read \n\nthe first RECEIVE_BYTE_COUNT bytes in \n\nthis register. This is the number of bytes \n\nin the RX_BUFFER_DATA_OBJECTS plus \n\nthree (for the RX_BUF_FRAME_TYPE and \n\nRX_BUF_HEADER)."]
    #[inline(always)]
    pub fn receive_byte_count(&self) -> ReceiveByteCountR {
        ReceiveByteCountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Receive Detect Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`receive_byte_count::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReceiveByteCountSpec;
impl crate::RegisterSpec for ReceiveByteCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`receive_byte_count::R`](R) reader structure"]
impl crate::Readable for ReceiveByteCountSpec {}
#[doc = "`reset()` method sets RECEIVE_BYTE_COUNT to value 0"]
impl crate::Resettable for ReceiveByteCountSpec {
    const RESET_VALUE: u32 = 0;
}
