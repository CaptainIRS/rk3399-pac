#[doc = "Register `EMMCCORE_ADMAERRSTS` reader"]
pub type R = crate::R<EmmccoreAdmaerrstsSpec>;
#[doc = "This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates \"10\" because ADMA never stops in this state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Admaerrorstate {
    #[doc = "0: ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    D0 = 0,
    #[doc = "1: ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    D1 = 1,
    #[doc = "2: ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    D2 = 2,
    #[doc = "3: ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    D3 = 3,
}
impl From<Admaerrorstate> for u8 {
    #[inline(always)]
    fn from(variant: Admaerrorstate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Admaerrorstate {
    type Ux = u8;
}
#[doc = "Field `ADMAERRORSTATE` reader - This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates \"10\" because ADMA never stops in this state."]
pub type AdmaerrorstateR = crate::FieldReader<Admaerrorstate>;
impl AdmaerrorstateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admaerrorstate {
        match self.bits {
            0 => Admaerrorstate::D0,
            1 => Admaerrorstate::D1,
            2 => Admaerrorstate::D2,
            3 => Admaerrorstate::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Admaerrorstate::D0
    }
    #[doc = "ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Admaerrorstate::D1
    }
    #[doc = "ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Admaerrorstate::D2
    }
    #[doc = "ST_TFR (Transfer Data) Points to the next of the error descriptor"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Admaerrorstate::D3
    }
}
#[doc = "ADMA Length Mismatch Error. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lenmismatch {
    #[doc = "1: No error"]
    B1 = 1,
    #[doc = "0: No error"]
    B0 = 0,
}
impl From<Lenmismatch> for bool {
    #[inline(always)]
    fn from(variant: Lenmismatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LENMISMATCH` reader - ADMA Length Mismatch Error. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
pub type LenmismatchR = crate::BitReader<Lenmismatch>;
impl LenmismatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lenmismatch {
        match self.bits {
            true => Lenmismatch::B1,
            false => Lenmismatch::B0,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lenmismatch::B1
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lenmismatch::B0
    }
}
impl R {
    #[doc = "Bits 0:1 - This field indicates the state of ADMA when error is occurred during ADMA data transfer. This field never indicates \"10\" because ADMA never stops in this state."]
    #[inline(always)]
    pub fn admaerrorstate(&self) -> AdmaerrorstateR {
        AdmaerrorstateR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error. While Block Count Enable being set, the total data length specified by the Descriptor table is different from that specified by the Block Count and Block Length. Total data length can not be divided by the block length."]
    #[inline(always)]
    pub fn lenmismatch(&self) -> LenmismatchR {
        LenmismatchR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "ADMA error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emmccore_admaerrsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmmccoreAdmaerrstsSpec;
impl crate::RegisterSpec for EmmccoreAdmaerrstsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`emmccore_admaerrsts::R`](R) reader structure"]
impl crate::Readable for EmmccoreAdmaerrstsSpec {}
#[doc = "`reset()` method sets EMMCCORE_ADMAERRSTS to value 0"]
impl crate::Resettable for EmmccoreAdmaerrstsSpec {
    const RESET_VALUE: u16 = 0;
}
