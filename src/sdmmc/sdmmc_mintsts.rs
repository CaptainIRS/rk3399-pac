#[doc = "Register `SDMMC_MINTSTS` reader"]
pub type R = crate::R<SdmmcMintstsSpec>;
#[doc = "Register `SDMMC_MINTSTS` writer"]
pub type W = crate::W<SdmmcMintstsSpec>;
#[doc = "Field `INT_STATUS` reader - Interrupt enabled only if corresponding bit in interrupt mask register is set. \\[15\\]: End-bit error (read)/Write no CRC (EBE) \\[14\\]: Auto command done (ACD) \\[13\\]: Start-bit error (SBE) \\[12\\]: Hardware locked write error (HLE) \\[11\\]: FIFO underrun/overrun error (FRUN) \\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int \\[9\\]: Data read timeout (DRTO) \\[8\\]: Response timeout (RTO) \\[7\\]: Data CRC error (DCRC) \\[6\\]: Response CRC error (RCRC) \\[5\\]: Receive FIFO data request (RXDR) \\[4\\]: Transmit FIFO data request (TXDR) \\[3\\]: Data transfer over (DTO) \\[2\\]: Command done (CD) \\[1\\]: Response error (RE) \\[0\\]: Card detect (CD)"]
pub type IntStatusR = crate::FieldReader<u16>;
#[doc = "Field `DATA_NOBUSY_INT_STATUS` reader - Data no busy Interrupt Status"]
pub type DataNobusyIntStatusR = crate::BitReader;
#[doc = "Field `DATA_NOBUSY_INT_STATUS` writer - Data no busy Interrupt Status"]
pub type DataNobusyIntStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt from SDIO card; SDIO interrupt for card enabled only if corresponding sdio_int_mask bit is set in Interrupt mask register (mask bit 1 enables interrupt; 0 masks interrupt).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioInterrupt {
    #[doc = "0: SDIO interrupt from card"]
    B0 = 0,
    #[doc = "1: SDIO interrupt from card"]
    B1 = 1,
}
impl From<SdioInterrupt> for bool {
    #[inline(always)]
    fn from(variant: SdioInterrupt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDIO_INTERRUPT` reader - Interrupt from SDIO card; SDIO interrupt for card enabled only if corresponding sdio_int_mask bit is set in Interrupt mask register (mask bit 1 enables interrupt; 0 masks interrupt)."]
pub type SdioInterruptR = crate::BitReader<SdioInterrupt>;
impl SdioInterruptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SdioInterrupt {
        match self.bits {
            false => SdioInterrupt::B0,
            true => SdioInterrupt::B1,
        }
    }
    #[doc = "SDIO interrupt from card"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SdioInterrupt::B0
    }
    #[doc = "SDIO interrupt from card"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SdioInterrupt::B1
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt enabled only if corresponding bit in interrupt mask register is set. \\[15\\]: End-bit error (read)/Write no CRC (EBE) \\[14\\]: Auto command done (ACD) \\[13\\]: Start-bit error (SBE) \\[12\\]: Hardware locked write error (HLE) \\[11\\]: FIFO underrun/overrun error (FRUN) \\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int \\[9\\]: Data read timeout (DRTO) \\[8\\]: Response timeout (RTO) \\[7\\]: Data CRC error (DCRC) \\[6\\]: Response CRC error (RCRC) \\[5\\]: Receive FIFO data request (RXDR) \\[4\\]: Transmit FIFO data request (TXDR) \\[3\\]: Data transfer over (DTO) \\[2\\]: Command done (CD) \\[1\\]: Response error (RE) \\[0\\]: Card detect (CD)"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Data no busy Interrupt Status"]
    #[inline(always)]
    pub fn data_nobusy_int_status(&self) -> DataNobusyIntStatusR {
        DataNobusyIntStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt from SDIO card; SDIO interrupt for card enabled only if corresponding sdio_int_mask bit is set in Interrupt mask register (mask bit 1 enables interrupt; 0 masks interrupt)."]
    #[inline(always)]
    pub fn sdio_interrupt(&self) -> SdioInterruptR {
        SdioInterruptR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Data no busy Interrupt Status"]
    #[inline(always)]
    #[must_use]
    pub fn data_nobusy_int_status(&mut self) -> DataNobusyIntStatusW<SdmmcMintstsSpec> {
        DataNobusyIntStatusW::new(self, 16)
    }
}
#[doc = "Masked interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_mintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_mintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmmcMintstsSpec;
impl crate::RegisterSpec for SdmmcMintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_mintsts::R`](R) reader structure"]
impl crate::Readable for SdmmcMintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_mintsts::W`](W) writer structure"]
impl crate::Writable for SdmmcMintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_MINTSTS to value 0"]
impl crate::Resettable for SdmmcMintstsSpec {
    const RESET_VALUE: u32 = 0;
}
