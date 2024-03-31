#[doc = "Register `RINTSTS` reader"]
pub type R = crate::R<RintstsSpec>;
#[doc = "Register `RINTSTS` writer"]
pub type W = crate::W<RintstsSpec>;
#[doc = "Field `INT_STATUS` reader - Writes to bits clear status bit. Value of 1 clears status bit, and\n\nvalue of 0 leaves bit intact. Bits are logged regardless of interrupt\n\nmask status.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
pub type IntStatusR = crate::FieldReader<u16>;
#[doc = "Field `DATA_NOBUSY_INT_STATUS` reader - Data no busy interrupt status"]
pub type DataNobusyIntStatusR = crate::BitReader;
#[doc = "Field `DATA_NOBUSY_INT_STATUS` writer - Data no busy interrupt status"]
pub type DataNobusyIntStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Interrupt from SDIO card; Writes to these bits clear them. Value\n\nof 1 clears bit and 0 leaves bit intact.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdioInterrupt {
    #[doc = "0: no SDIO interrupt from card"]
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
#[doc = "Field `SDIO_INTERRUPT` reader - Interrupt from SDIO card; Writes to these bits clear them. Value\n\nof 1 clears bit and 0 leaves bit intact."]
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
    #[doc = "no SDIO interrupt from card"]
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
    #[doc = "Bits 0:15 - Writes to bits clear status bit. Value of 1 clears status bit, and\n\nvalue of 0 leaves bit intact. Bits are logged regardless of interrupt\n\nmask status.\n\n\\[15\\]: End-bit error (read)/Write no CRC (EBE)\n\n\\[14\\]: Auto command done (ACD)\n\n\\[13\\]: Start-bit error (SBE)\n\n\\[12\\]: Hardware locked write error (HLE)\n\n\\[11\\]: FIFO underrun/overrun error (FRUN)\n\n\\[10\\]: Data starvation-by-host timeout (HTO) /Volt_switch_int\n\n\\[9\\]: Data read timeout (DRTO)\n\n\\[8\\]: Response timeout (RTO)\n\n\\[7\\]: Data CRC error (DCRC)\n\n\\[6\\]: Response CRC error (RCRC)\n\n\\[5\\]: Receive FIFO data request (RXDR)\n\n\\[4\\]: Transmit FIFO data request (TXDR)\n\n\\[3\\]: Data transfer over (DTO)\n\n\\[2\\]: Command done (CD)\n\n\\[1\\]: Response error (RE)\n\n\\[0\\]: Card detect (CD)"]
    #[inline(always)]
    pub fn int_status(&self) -> IntStatusR {
        IntStatusR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Data no busy interrupt status"]
    #[inline(always)]
    pub fn data_nobusy_int_status(&self) -> DataNobusyIntStatusR {
        DataNobusyIntStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt from SDIO card; Writes to these bits clear them. Value\n\nof 1 clears bit and 0 leaves bit intact."]
    #[inline(always)]
    pub fn sdio_interrupt(&self) -> SdioInterruptR {
        SdioInterruptR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Data no busy interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn data_nobusy_int_status(&mut self) -> DataNobusyIntStatusW<RintstsSpec> {
        DataNobusyIntStatusW::new(self, 16)
    }
}
#[doc = "Raw interrupt-status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RintstsSpec;
impl crate::RegisterSpec for RintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rintsts::R`](R) reader structure"]
impl crate::Readable for RintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`rintsts::W`](W) writer structure"]
impl crate::Writable for RintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINTSTS to value 0"]
impl crate::Resettable for RintstsSpec {
    const RESET_VALUE: u32 = 0;
}
