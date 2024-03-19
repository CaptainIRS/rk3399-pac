#[doc = "Register `I2S_RXFIFOLR` reader"]
pub type R = crate::R<I2sRxfifolrSpec>;
#[doc = "Field `RFL0` reader - Receive FIFO0 Level\n\nContains the number of valid data entries in the receive FIFO0."]
pub type Rfl0R = crate::FieldReader;
#[doc = "Field `RFL1` reader - Receive FIFO1 Level\n\nContains the number of valid data entries in the receive FIFO1."]
pub type Rfl1R = crate::FieldReader;
#[doc = "Field `RFL2` reader - Receive FIFO2 Level\n\nContains the number of valid data entries in the receive FIFO2."]
pub type Rfl2R = crate::FieldReader;
#[doc = "Field `RFL3` reader - Receive FIFO3 Level\n\nContains the number of valid data entries in the receive FIFO3."]
pub type Rfl3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Receive FIFO0 Level\n\nContains the number of valid data entries in the receive FIFO0."]
    #[inline(always)]
    pub fn rfl0(&self) -> Rfl0R {
        Rfl0R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Receive FIFO1 Level\n\nContains the number of valid data entries in the receive FIFO1."]
    #[inline(always)]
    pub fn rfl1(&self) -> Rfl1R {
        Rfl1R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Receive FIFO2 Level\n\nContains the number of valid data entries in the receive FIFO2."]
    #[inline(always)]
    pub fn rfl2(&self) -> Rfl2R {
        Rfl2R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Receive FIFO3 Level\n\nContains the number of valid data entries in the receive FIFO3."]
    #[inline(always)]
    pub fn rfl3(&self) -> Rfl3R {
        Rfl3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[doc = "RX FIFO level register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_rxfifolr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sRxfifolrSpec;
impl crate::RegisterSpec for I2sRxfifolrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_rxfifolr::R`](R) reader structure"]
impl crate::Readable for I2sRxfifolrSpec {}
#[doc = "`reset()` method sets I2S_RXFIFOLR to value 0"]
impl crate::Resettable for I2sRxfifolrSpec {
    const RESET_VALUE: u32 = 0;
}
