#[doc = "Register `ErrLog0` reader"]
pub type R = crate::R<ErrLog0Spec>;
#[doc = "Field `LOCK` reader - Contains packet header bit Lock."]
pub type LockR = crate::BitReader;
#[doc = "Field `OPC` reader - Contains packet header field Opc."]
pub type OpcR = crate::FieldReader;
#[doc = "Field `ERRCODE` reader - Contains packet header field ErrCode if the field exists, otherwise 0."]
pub type ErrcodeR = crate::FieldReader;
#[doc = "Field `LEN1` reader - Contains packet header field Len1"]
pub type Len1R = crate::FieldReader<u16>;
#[doc = "Field `FORMAT` reader - Always 1."]
pub type FormatR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Contains packet header bit Lock."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Contains packet header field Opc."]
    #[inline(always)]
    pub fn opc(&self) -> OpcR {
        OpcR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Contains packet header field ErrCode if the field exists, otherwise 0."]
    #[inline(always)]
    pub fn errcode(&self) -> ErrcodeR {
        ErrcodeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:27 - Contains packet header field Len1"]
    #[inline(always)]
    pub fn len1(&self) -> Len1R {
        Len1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Always 1."]
    #[inline(always)]
    pub fn format(&self) -> FormatR {
        FormatR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Transport protocol header information register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`err_log0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrLog0Spec;
impl crate::RegisterSpec for ErrLog0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_log0::R`](R) reader structure"]
impl crate::Readable for ErrLog0Spec {}
#[doc = "`reset()` method sets ErrLog0 to value 0x8000_0000"]
impl crate::Resettable for ErrLog0Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
