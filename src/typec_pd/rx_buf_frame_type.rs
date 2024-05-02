#[doc = "Register `RX_BUF_FRAME_TYPE` reader"]
pub type R = crate::R<RxBufFrameTypeSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ReceivedSopMessage {
    #[doc = "0: Received SOP."]
    B000 = 0,
    #[doc = "1: Received SOP’"]
    B001 = 1,
    #[doc = "2: Received SOP’’"]
    B010 = 2,
    #[doc = "3: Received SOP_DBG’"]
    B011 = 3,
    #[doc = "4: Received SOP_DBG’’"]
    B100 = 4,
    #[doc = "6: Received Cable Reset All others are reserved."]
    B110 = 6,
}
impl From<ReceivedSopMessage> for u8 {
    #[inline(always)]
    fn from(variant: ReceivedSopMessage) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReceivedSopMessage {
    type Ux = u8;
}
#[doc = "Field `Received_SOP_Message` reader - "]
pub type ReceivedSopMessageR = crate::FieldReader<ReceivedSopMessage>;
impl ReceivedSopMessageR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReceivedSopMessage> {
        match self.bits {
            0 => Some(ReceivedSopMessage::B000),
            1 => Some(ReceivedSopMessage::B001),
            2 => Some(ReceivedSopMessage::B010),
            3 => Some(ReceivedSopMessage::B011),
            4 => Some(ReceivedSopMessage::B100),
            6 => Some(ReceivedSopMessage::B110),
            _ => None,
        }
    }
    #[doc = "Received SOP."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == ReceivedSopMessage::B000
    }
    #[doc = "Received SOP’"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == ReceivedSopMessage::B001
    }
    #[doc = "Received SOP’’"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == ReceivedSopMessage::B010
    }
    #[doc = "Received SOP_DBG’"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == ReceivedSopMessage::B011
    }
    #[doc = "Received SOP_DBG’’"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == ReceivedSopMessage::B100
    }
    #[doc = "Received Cable Reset All others are reserved."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == ReceivedSopMessage::B110
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
pub type NotUsedR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn received_sop_message(&self) -> ReceivedSopMessageR {
        ReceivedSopMessageR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "RX Buffer Frame Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_buf_frame_type::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxBufFrameTypeSpec;
impl crate::RegisterSpec for RxBufFrameTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_buf_frame_type::R`](R) reader structure"]
impl crate::Readable for RxBufFrameTypeSpec {}
#[doc = "`reset()` method sets RX_BUF_FRAME_TYPE to value 0"]
impl crate::Resettable for RxBufFrameTypeSpec {
    const RESET_VALUE: u32 = 0;
}
