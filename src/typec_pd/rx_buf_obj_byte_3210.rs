#[doc = "Register `RX_BUF_OBJ%s_BYTE_3210` reader"]
pub type R = crate::R<RxBufObjByte3210Spec>;
#[doc = "Field `RX_BUF_OBJX_BYTE_0` reader - Byte 0 (bits 7..0) of X data object"]
pub type RxBufObjxByte0R = crate::FieldReader;
#[doc = "Field `RX_BUF_OBJX_BYTE_1` reader - Byte 1 (bits 15..8) of X data object"]
pub type RxBufObjxByte1R = crate::FieldReader;
#[doc = "Field `RX_BUF_OBJX_BYTE_2` reader - Byte 2 (bits 23..16) of X data object"]
pub type RxBufObjxByte2R = crate::FieldReader;
#[doc = "Field `RX_BUF_OBJX_BYTE_3` reader - Byte 3 (bits 31..24) of X data object"]
pub type RxBufObjxByte3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Byte 0 (bits 7..0) of X data object"]
    #[inline(always)]
    pub fn rx_buf_objx_byte_0(&self) -> RxBufObjxByte0R {
        RxBufObjxByte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 1 (bits 15..8) of X data object"]
    #[inline(always)]
    pub fn rx_buf_objx_byte_1(&self) -> RxBufObjxByte1R {
        RxBufObjxByte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte 2 (bits 23..16) of X data object"]
    #[inline(always)]
    pub fn rx_buf_objx_byte_2(&self) -> RxBufObjxByte2R {
        RxBufObjxByte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Byte 3 (bits 31..24) of X data object"]
    #[inline(always)]
    pub fn rx_buf_objx_byte_3(&self) -> RxBufObjxByte3R {
        RxBufObjxByte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RX Buffer OBJX Byte3~0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_buf_obj_byte_3210::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxBufObjByte3210Spec;
impl crate::RegisterSpec for RxBufObjByte3210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_buf_obj_byte_3210::R`](R) reader structure"]
impl crate::Readable for RxBufObjByte3210Spec {}
#[doc = "`reset()` method sets RX_BUF_OBJ%s_BYTE_3210 to value 0"]
impl crate::Resettable for RxBufObjByte3210Spec {
    const RESET_VALUE: u32 = 0;
}
