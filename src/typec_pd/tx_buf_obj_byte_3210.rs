#[doc = "Register `TX_BUF_OBJ%s_BYTE_3210` reader"]
pub type R = crate::R<TxBufObjByte3210Spec>;
#[doc = "Register `TX_BUF_OBJ%s_BYTE_3210` writer"]
pub type W = crate::W<TxBufObjByte3210Spec>;
#[doc = "Field `TX_BUF_OBJX_BYTE_0` reader - Byte 0 (bits 7..0) of X data object"]
pub type TxBufObjxByte0R = crate::FieldReader;
#[doc = "Field `TX_BUF_OBJX_BYTE_0` writer - Byte 0 (bits 7..0) of X data object"]
pub type TxBufObjxByte0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_BUF_OBJX_BYTE_1` reader - Byte 1 (bits 15..8) of X data object"]
pub type TxBufObjxByte1R = crate::FieldReader;
#[doc = "Field `TX_BUF_OBJX_BYTE_1` writer - Byte 1 (bits 15..8) of X data object"]
pub type TxBufObjxByte1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_BUF_OBJX_BYTE_2` reader - Byte 2 (bits 23..16) of X data object"]
pub type TxBufObjxByte2R = crate::FieldReader;
#[doc = "Field `TX_BUF_OBJX_BYTE_2` writer - Byte 2 (bits 23..16) of X data object"]
pub type TxBufObjxByte2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_BUF_OBJX_BYTE_3` reader - Byte 3 (bits 31..24) of X data object"]
pub type TxBufObjxByte3R = crate::FieldReader;
#[doc = "Field `TX_BUF_OBJX_BYTE_3` writer - Byte 3 (bits 31..24) of X data object"]
pub type TxBufObjxByte3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Byte 0 (bits 7..0) of X data object"]
    #[inline(always)]
    pub fn tx_buf_objx_byte_0(&self) -> TxBufObjxByte0R {
        TxBufObjxByte0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Byte 1 (bits 15..8) of X data object"]
    #[inline(always)]
    pub fn tx_buf_objx_byte_1(&self) -> TxBufObjxByte1R {
        TxBufObjxByte1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Byte 2 (bits 23..16) of X data object"]
    #[inline(always)]
    pub fn tx_buf_objx_byte_2(&self) -> TxBufObjxByte2R {
        TxBufObjxByte2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Byte 3 (bits 31..24) of X data object"]
    #[inline(always)]
    pub fn tx_buf_objx_byte_3(&self) -> TxBufObjxByte3R {
        TxBufObjxByte3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte 0 (bits 7..0) of X data object"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_objx_byte_0(&mut self) -> TxBufObjxByte0W<TxBufObjByte3210Spec> {
        TxBufObjxByte0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Byte 1 (bits 15..8) of X data object"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_objx_byte_1(&mut self) -> TxBufObjxByte1W<TxBufObjByte3210Spec> {
        TxBufObjxByte1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Byte 2 (bits 23..16) of X data object"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_objx_byte_2(&mut self) -> TxBufObjxByte2W<TxBufObjByte3210Spec> {
        TxBufObjxByte2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Byte 3 (bits 31..24) of X data object"]
    #[inline(always)]
    #[must_use]
    pub fn tx_buf_objx_byte_3(&mut self) -> TxBufObjxByte3W<TxBufObjByte3210Spec> {
        TxBufObjxByte3W::new(self, 24)
    }
}
#[doc = "TX Buffer OBJX Byte3~0 Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_buf_obj_byte_3210::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_buf_obj_byte_3210::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxBufObjByte3210Spec;
impl crate::RegisterSpec for TxBufObjByte3210Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_buf_obj_byte_3210::R`](R) reader structure"]
impl crate::Readable for TxBufObjByte3210Spec {}
#[doc = "`write(|w| ..)` method takes [`tx_buf_obj_byte_3210::W`](W) writer structure"]
impl crate::Writable for TxBufObjByte3210Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_BUF_OBJ%s_BYTE_3210 to value 0"]
impl crate::Resettable for TxBufObjByte3210Spec {
    const RESET_VALUE: u32 = 0;
}
