#[doc = "Register `PIPE_CMN_CTRL1` reader"]
pub type R = crate::R<PipeCmnCtrl1Spec>;
#[doc = "Register `PIPE_CMN_CTRL1` writer"]
pub type W = crate::W<PipeCmnCtrl1Spec>;
#[doc = "Field `FIELD7` reader - USB Elasticity Buffer Re-align enable : When this bit is set to 1, \n\nwhen RX for a USB link is started, the elasticity buffer is re-aligned to \n\nits idle point upon seeing the 3 consective COMMAs in the same \n\nrelative bit position (first instance only). This will re-align the elasticity \n\nbuffer (i.e. CTC FIFO) after receiving the TSEQ data which contains no \n\nSKIP OSs."]
pub type Field7R = crate::BitReader;
#[doc = "Field `FIELD7` writer - USB Elasticity Buffer Re-align enable : When this bit is set to 1, \n\nwhen RX for a USB link is started, the elasticity buffer is re-aligned to \n\nits idle point upon seeing the 3 consective COMMAs in the same \n\nrelative bit position (first instance only). This will re-align the elasticity \n\nbuffer (i.e. CTC FIFO) after receiving the TSEQ data which contains no \n\nSKIP OSs."]
pub type Field7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD6` reader - USB Loopback Slave Error Count disable : When this bit is set \n\nto 1, the error count insertion for USB loopback slave is disabled, \n\nsuch that the error count is not inserted into the BCNT OS."]
pub type Field6R = crate::BitReader;
#[doc = "Field `FIELD6` writer - USB Loopback Slave Error Count disable : When this bit is set \n\nto 1, the error count insertion for USB loopback slave is disabled, \n\nsuch that the error count is not inserted into the BCNT OS."]
pub type Field6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD5` reader - USB PIPE3 Compatibility Mode enable : When this bit is set to 1, \n\nUSB PIPE3 compatibility mode \n\nis enabled. In \n\nthis mode, when \n\noperating in nominal empty Elasticity Buffer mode, when the EB buffer \n\ngoes empty, instead of de-asserting PIPE RxDataValid, a USB SKIP OS \n\nis inserted into the data stream. This is the behavior as defined in PIPE \n\nversion 3. When this bit is low, PIPE RxDataValid is de-asserted when \n\nthe EB buffer goes empty, as recommended by PIPE version 4."]
pub type Field5R = crate::BitReader;
#[doc = "Field `FIELD5` writer - USB PIPE3 Compatibility Mode enable : When this bit is set to 1, \n\nUSB PIPE3 compatibility mode \n\nis enabled. In \n\nthis mode, when \n\noperating in nominal empty Elasticity Buffer mode, when the EB buffer \n\ngoes empty, instead of de-asserting PIPE RxDataValid, a USB SKIP OS \n\nis inserted into the data stream. This is the behavior as defined in PIPE \n\nversion 3. When this bit is low, PIPE RxDataValid is de-asserted when \n\nthe EB buffer goes empty, as recommended by PIPE version 4."]
pub type Field5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD4` reader - Reserved"]
pub type Field4R = crate::BitReader;
#[doc = "Field `FIELD3` reader - TX electrical idle pre release : When this bit is set, the TX electrical \n\nidle release to the PMA is advanced 1 cycle to allow the adjustment of \n\nthe datapath timing."]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD3` writer - TX electrical idle pre release : When this bit is set, the TX electrical \n\nidle release to the PMA is advanced 1 cycle to allow the adjustment of \n\nthe datapath timing."]
pub type Field3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD2` reader - Reserved"]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD1` reader - Comma realign: This field controls the comma alignment state \n\nmachine to re-align to new bit position without going to loss of sync \n\nstate. The requirement of the new bit position should meet the \n\nnumber of COMMAs as per Symbol unlock count register definition. \n\nWhen new bit position \n\nis \n\nidentified the comma alignment state \n\nmachine remains in sync state with the alignment now locked to the \n\nnew bit position. This field needs to be programmed during the PHY \n\ninitialization routine before training sequences are received. The effect \n\nhere is that pipe_rx_valid is not de-asserted upon re-alignment. When \n\nthis bit is 0, pipe_rx_valid will be de-asserted upon loss of COMMA \n\nlock and subsequent re-alignment."]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD1` writer - Comma realign: This field controls the comma alignment state \n\nmachine to re-align to new bit position without going to loss of sync \n\nstate. The requirement of the new bit position should meet the \n\nnumber of COMMAs as per Symbol unlock count register definition. \n\nWhen new bit position \n\nis \n\nidentified the comma alignment state \n\nmachine remains in sync state with the alignment now locked to the \n\nnew bit position. This field needs to be programmed during the PHY \n\ninitialization routine before training sequences are received. The effect \n\nhere is that pipe_rx_valid is not de-asserted upon re-alignment. When \n\nthis bit is 0, pipe_rx_valid will be de-asserted upon loss of COMMA \n\nlock and subsequent re-alignment."]
pub type Field1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD0` reader - Reserved"]
pub type Field0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - USB Elasticity Buffer Re-align enable : When this bit is set to 1, \n\nwhen RX for a USB link is started, the elasticity buffer is re-aligned to \n\nits idle point upon seeing the 3 consective COMMAs in the same \n\nrelative bit position (first instance only). This will re-align the elasticity \n\nbuffer (i.e. CTC FIFO) after receiving the TSEQ data which contains no \n\nSKIP OSs."]
    #[inline(always)]
    pub fn field7(&self) -> Field7R {
        Field7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Loopback Slave Error Count disable : When this bit is set \n\nto 1, the error count insertion for USB loopback slave is disabled, \n\nsuch that the error count is not inserted into the BCNT OS."]
    #[inline(always)]
    pub fn field6(&self) -> Field6R {
        Field6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB PIPE3 Compatibility Mode enable : When this bit is set to 1, \n\nUSB PIPE3 compatibility mode \n\nis enabled. In \n\nthis mode, when \n\noperating in nominal empty Elasticity Buffer mode, when the EB buffer \n\ngoes empty, instead of de-asserting PIPE RxDataValid, a USB SKIP OS \n\nis inserted into the data stream. This is the behavior as defined in PIPE \n\nversion 3. When this bit is low, PIPE RxDataValid is de-asserted when \n\nthe EB buffer goes empty, as recommended by PIPE version 4."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX electrical idle pre release : When this bit is set, the TX electrical \n\nidle release to the PMA is advanced 1 cycle to allow the adjustment of \n\nthe datapath timing."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Reserved"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Comma realign: This field controls the comma alignment state \n\nmachine to re-align to new bit position without going to loss of sync \n\nstate. The requirement of the new bit position should meet the \n\nnumber of COMMAs as per Symbol unlock count register definition. \n\nWhen new bit position \n\nis \n\nidentified the comma alignment state \n\nmachine remains in sync state with the alignment now locked to the \n\nnew bit position. This field needs to be programmed during the PHY \n\ninitialization routine before training sequences are received. The effect \n\nhere is that pipe_rx_valid is not de-asserted upon re-alignment. When \n\nthis bit is 0, pipe_rx_valid will be de-asserted upon loss of COMMA \n\nlock and subsequent re-alignment."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 9) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB Elasticity Buffer Re-align enable : When this bit is set to 1, \n\nwhen RX for a USB link is started, the elasticity buffer is re-aligned to \n\nits idle point upon seeing the 3 consective COMMAs in the same \n\nrelative bit position (first instance only). This will re-align the elasticity \n\nbuffer (i.e. CTC FIFO) after receiving the TSEQ data which contains no \n\nSKIP OSs."]
    #[inline(always)]
    #[must_use]
    pub fn field7(&mut self) -> Field7W<PipeCmnCtrl1Spec> {
        Field7W::new(self, 0)
    }
    #[doc = "Bit 1 - USB Loopback Slave Error Count disable : When this bit is set \n\nto 1, the error count insertion for USB loopback slave is disabled, \n\nsuch that the error count is not inserted into the BCNT OS."]
    #[inline(always)]
    #[must_use]
    pub fn field6(&mut self) -> Field6W<PipeCmnCtrl1Spec> {
        Field6W::new(self, 1)
    }
    #[doc = "Bit 2 - USB PIPE3 Compatibility Mode enable : When this bit is set to 1, \n\nUSB PIPE3 compatibility mode \n\nis enabled. In \n\nthis mode, when \n\noperating in nominal empty Elasticity Buffer mode, when the EB buffer \n\ngoes empty, instead of de-asserting PIPE RxDataValid, a USB SKIP OS \n\nis inserted into the data stream. This is the behavior as defined in PIPE \n\nversion 3. When this bit is low, PIPE RxDataValid is de-asserted when \n\nthe EB buffer goes empty, as recommended by PIPE version 4."]
    #[inline(always)]
    #[must_use]
    pub fn field5(&mut self) -> Field5W<PipeCmnCtrl1Spec> {
        Field5W::new(self, 2)
    }
    #[doc = "Bit 4 - TX electrical idle pre release : When this bit is set, the TX electrical \n\nidle release to the PMA is advanced 1 cycle to allow the adjustment of \n\nthe datapath timing."]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PipeCmnCtrl1Spec> {
        Field3W::new(self, 4)
    }
    #[doc = "Bit 8 - Comma realign: This field controls the comma alignment state \n\nmachine to re-align to new bit position without going to loss of sync \n\nstate. The requirement of the new bit position should meet the \n\nnumber of COMMAs as per Symbol unlock count register definition. \n\nWhen new bit position \n\nis \n\nidentified the comma alignment state \n\nmachine remains in sync state with the alignment now locked to the \n\nnew bit position. This field needs to be programmed during the PHY \n\ninitialization routine before training sequences are received. The effect \n\nhere is that pipe_rx_valid is not de-asserted upon re-alignment. When \n\nthis bit is 0, pipe_rx_valid will be de-asserted upon loss of COMMA \n\nlock and subsequent re-alignment."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<PipeCmnCtrl1Spec> {
        Field1W::new(self, 8)
    }
}
#[doc = "PIPE common control1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipe_cmn_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipe_cmn_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PipeCmnCtrl1Spec;
impl crate::RegisterSpec for PipeCmnCtrl1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipe_cmn_ctrl1::R`](R) reader structure"]
impl crate::Readable for PipeCmnCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pipe_cmn_ctrl1::W`](W) writer structure"]
impl crate::Writable for PipeCmnCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PIPE_CMN_CTRL1 to value 0x0111"]
impl crate::Resettable for PipeCmnCtrl1Spec {
    const RESET_VALUE: u16 = 0x0111;
}
