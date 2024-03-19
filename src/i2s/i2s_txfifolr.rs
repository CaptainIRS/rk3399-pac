#[doc = "Register `I2S_TXFIFOLR` reader"]
pub type R = crate::R<I2sTxfifolrSpec>;
#[doc = "Field `TFL0` reader - Transmit FIFO0 Level\n\nContains the number of valid data entries in the transmit FIFO0."]
pub type Tfl0R = crate::FieldReader;
#[doc = "Field `TFL1` reader - Transmit FIFO1 Level\n\nContains the number of valid data entries in the transmit FIFO1."]
pub type Tfl1R = crate::FieldReader;
#[doc = "Field `TFL2` reader - Transmit FIFO2 Level\n\nContains the number of valid data entries in the transmit FIFO2."]
pub type Tfl2R = crate::FieldReader;
#[doc = "Field `TFL3` reader - Transmit FIFO3 Level\n\nContains the number of valid data entries in the transmit FIFO3."]
pub type Tfl3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO0 Level\n\nContains the number of valid data entries in the transmit FIFO0."]
    #[inline(always)]
    pub fn tfl0(&self) -> Tfl0R {
        Tfl0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Transmit FIFO1 Level\n\nContains the number of valid data entries in the transmit FIFO1."]
    #[inline(always)]
    pub fn tfl1(&self) -> Tfl1R {
        Tfl1R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Transmit FIFO2 Level\n\nContains the number of valid data entries in the transmit FIFO2."]
    #[inline(always)]
    pub fn tfl2(&self) -> Tfl2R {
        Tfl2R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Transmit FIFO3 Level\n\nContains the number of valid data entries in the transmit FIFO3."]
    #[inline(always)]
    pub fn tfl3(&self) -> Tfl3R {
        Tfl3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[doc = "TX FIFO level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_txfifolr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sTxfifolrSpec;
impl crate::RegisterSpec for I2sTxfifolrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_txfifolr::R`](R) reader structure"]
impl crate::Readable for I2sTxfifolrSpec {}
#[doc = "`reset()` method sets I2S_TXFIFOLR to value 0"]
impl crate::Resettable for I2sTxfifolrSpec {
    const RESET_VALUE: u32 = 0;
}
