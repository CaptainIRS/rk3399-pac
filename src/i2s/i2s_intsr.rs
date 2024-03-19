#[doc = "Register `I2S_INTSR` reader"]
pub type R = crate::R<I2sIntsrSpec>;
#[doc = "TX empty interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txei {
    #[doc = "0: inactive"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Txei> for bool {
    #[inline(always)]
    fn from(variant: Txei) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEI` reader - TX empty interrupt"]
pub type TxeiR = crate::BitReader<Txei>;
impl TxeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txei {
        match self.bits {
            false => Txei::B0,
            true => Txei::B1,
        }
    }
    #[doc = "inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txei::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txei::B1
    }
}
#[doc = "TX underrun interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txui {
    #[doc = "0: inactive"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Txui> for bool {
    #[inline(always)]
    fn from(variant: Txui) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUI` reader - TX underrun interrupt"]
pub type TxuiR = crate::BitReader<Txui>;
impl TxuiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txui {
        match self.bits {
            false => Txui::B0,
            true => Txui::B1,
        }
    }
    #[doc = "inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txui::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txui::B1
    }
}
#[doc = "RX full interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfi {
    #[doc = "0: inactive"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Rxfi> for bool {
    #[inline(always)]
    fn from(variant: Rxfi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFI` reader - RX full interrupt"]
pub type RxfiR = crate::BitReader<Rxfi>;
impl RxfiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfi {
        match self.bits {
            false => Rxfi::B0,
            true => Rxfi::B1,
        }
    }
    #[doc = "inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxfi::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxfi::B1
    }
}
#[doc = "RX overrun interrupt\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxoi {
    #[doc = "0: inactive"]
    B0 = 0,
    #[doc = "1: active"]
    B1 = 1,
}
impl From<Rxoi> for bool {
    #[inline(always)]
    fn from(variant: Rxoi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOI` reader - RX overrun interrupt"]
pub type RxoiR = crate::BitReader<Rxoi>;
impl RxoiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxoi {
        match self.bits {
            false => Rxoi::B0,
            true => Rxoi::B1,
        }
    }
    #[doc = "inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxoi::B0
    }
    #[doc = "active"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxoi::B1
    }
}
impl R {
    #[doc = "Bit 0 - TX empty interrupt"]
    #[inline(always)]
    pub fn txei(&self) -> TxeiR {
        TxeiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX underrun interrupt"]
    #[inline(always)]
    pub fn txui(&self) -> TxuiR {
        TxuiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - RX full interrupt"]
    #[inline(always)]
    pub fn rxfi(&self) -> RxfiR {
        RxfiR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RX overrun interrupt"]
    #[inline(always)]
    pub fn rxoi(&self) -> RxoiR {
        RxoiR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_intsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sIntsrSpec;
impl crate::RegisterSpec for I2sIntsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_intsr::R`](R) reader structure"]
impl crate::Readable for I2sIntsrSpec {}
#[doc = "`reset()` method sets I2S_INTSR to value 0"]
impl crate::Resettable for I2sIntsrSpec {
    const RESET_VALUE: u32 = 0;
}
