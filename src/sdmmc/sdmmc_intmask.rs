#[doc = "Register `SDMMC_INTMASK` reader"]
pub type R = crate::R<SdmmcIntmaskSpec>;
#[doc = "Register `SDMMC_INTMASK` writer"]
pub type W = crate::W<SdmmcIntmaskSpec>;
#[doc = "Field `INT_MASK` reader - Bits used to mask unwanted interrupts. Value of 0 masks\n\ninterrupt; value of 1 enables interrupt.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
pub type IntMaskR = crate::FieldReader<u16>;
#[doc = "Field `INT_MASK` writer - Bits used to mask unwanted interrupts. Value of 0 masks\n\ninterrupt; value of 1 enables interrupt.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
pub type IntMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataNobusyIntMask {
    #[doc = "0: data no busy interrupt not masked"]
    B0 = 0,
    #[doc = "1: data no busy interrupt masked"]
    B1 = 1,
}
impl From<DataNobusyIntMask> for bool {
    #[inline(always)]
    fn from(variant: DataNobusyIntMask) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATA_NOBUSY_INT_MASK` reader - "]
pub type DataNobusyIntMaskR = crate::BitReader<DataNobusyIntMask>;
impl DataNobusyIntMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataNobusyIntMask {
        match self.bits {
            false => DataNobusyIntMask::B0,
            true => DataNobusyIntMask::B1,
        }
    }
    #[doc = "data no busy interrupt not masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DataNobusyIntMask::B0
    }
    #[doc = "data no busy interrupt masked"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DataNobusyIntMask::B1
    }
}
#[doc = "Field `DATA_NOBUSY_INT_MASK` writer - "]
pub type DataNobusyIntMaskW<'a, REG> = crate::BitWriter<'a, REG, DataNobusyIntMask>;
impl<'a, REG> DataNobusyIntMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "data no busy interrupt not masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DataNobusyIntMask::B0)
    }
    #[doc = "data no busy interrupt masked"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DataNobusyIntMask::B1)
    }
}
#[doc = "Field `SDIO_INT_MASK` reader - Mask SDIO interrupts.\n\nWhen masked, SDIO interrupt detection for that card is disabled.\n\nA 0 masks an interrupt, and 1 enables an interrupt."]
pub type SdioIntMaskR = crate::BitReader;
#[doc = "Field `SDIO_INT_MASK` writer - Mask SDIO interrupts.\n\nWhen masked, SDIO interrupt detection for that card is disabled.\n\nA 0 masks an interrupt, and 1 enables an interrupt."]
pub type SdioIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Bits used to mask unwanted interrupts. Value of 0 masks\n\ninterrupt; value of 1 enables interrupt.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
    #[inline(always)]
    pub fn int_mask(&self) -> IntMaskR {
        IntMaskR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn data_nobusy_int_mask(&self) -> DataNobusyIntMaskR {
        DataNobusyIntMaskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Mask SDIO interrupts.\n\nWhen masked, SDIO interrupt detection for that card is disabled.\n\nA 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SdioIntMaskR {
        SdioIntMaskR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bits used to mask unwanted interrupts. Value of 0 masks\n\ninterrupt; value of 1 enables interrupt.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
    #[inline(always)]
    #[must_use]
    pub fn int_mask(&mut self) -> IntMaskW<SdmmcIntmaskSpec> {
        IntMaskW::new(self, 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn data_nobusy_int_mask(&mut self) -> DataNobusyIntMaskW<SdmmcIntmaskSpec> {
        DataNobusyIntMaskW::new(self, 16)
    }
    #[doc = "Bit 24 - Mask SDIO interrupts.\n\nWhen masked, SDIO interrupt detection for that card is disabled.\n\nA 0 masks an interrupt, and 1 enables an interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_int_mask(&mut self) -> SdioIntMaskW<SdmmcIntmaskSpec> {
        SdioIntMaskW::new(self, 24)
    }
}
#[doc = "Interrupt-mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_intmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_intmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcIntmaskSpec;
impl crate::RegisterSpec for SdmmcIntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_intmask::R`](R) reader structure"]
impl crate::Readable for SdmmcIntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_intmask::W`](W) writer structure"]
impl crate::Writable for SdmmcIntmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_INTMASK to value 0"]
impl crate::Resettable for SdmmcIntmaskSpec {
    const RESET_VALUE: u32 = 0;
}
